package com.plugin.ahqstore

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.Intent
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import android.provider.Settings
import android.webkit.WebView
import androidx.core.content.FileProvider
import androidx.work.Constraints
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.NetworkType
import androidx.work.PeriodicWorkRequestBuilder
import androidx.work.WorkManager

import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

import ru.solrudev.ackpine.installer.PackageInstaller
import ru.solrudev.ackpine.installer.parameters.InstallParameters
import ru.solrudev.ackpine.session.Session
import ru.solrudev.ackpine.session.parameters.Confirmation
import ru.solrudev.ackpine.session.await

import ru.solrudev.ackpine.uninstaller.PackageUninstaller
import ru.solrudev.ackpine.uninstaller.parameters.UninstallParameters
import java.io.File
import java.util.Vector
import java.util.concurrent.TimeUnit
import kotlin.coroutines.cancellation.CancellationException

@InvokeArg
class ShowCodeRequest {
  var value: String = ""
}

@InvokeArg
class ZoomRequest {
  var zoom: Float = 100.0F
}

@InvokeArg
class Data {
  var data: String = ""
}

val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())

@TauriPlugin
class AHQStorePlugin(private val activity: Activity): Plugin(activity) {
    private var webView: WebView? = null
    private var pkgInstaller: PackageInstaller? = null
    private var pkgUninstaller: PackageUninstaller? = null

    override fun load(webView: WebView) {
      this.webView = webView

      if (!activity.shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS)) {
        activity.requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 0)
      }

      val manager = WorkManager.getInstance(activity.baseContext!!)

      val constraints = Constraints(
        requiresBatteryNotLow = true,
        requiredNetworkType = NetworkType.CONNECTED
      )

      val periodicWork = PeriodicWorkRequestBuilder<BackgroundUpdateWorker>(
        15,
        TimeUnit.MINUTES
      )
        .setConstraints(constraints)
        .build()

      manager.enqueueUniquePeriodicWork(
        "updater",
        ExistingPeriodicWorkPolicy.UPDATE,
        periodicWork
      )
    }

    @SuppressLint("QueryPermissionsNeeded")
    private fun loadAHQStoreApps(): Vector<PackageInfo> {
      val packageManager = activity.baseContext.packageManager

      val packages: Vector<PackageInfo> = Vector()

      for (application in packageManager.getInstalledPackages(0)) {
        val name = application!!.packageName

        if (isAHQStorePackage(name)) {
          packages.add(application)
        }
      }

      return packages
    }

    @Command
    fun install(invoke: Invoke) {
      scope.launch {
        installInner(invoke)
      }
    }

    @Command
    fun uninstall(invoke: Invoke) {
      scope.launch {
        uninstallInner(invoke)
      }
    }

    @Command
    fun getAndroidBuild(invoke: Invoke) {
      val ret = JSObject()

      ret.put("sdk", Build.VERSION.SDK_INT)
      ret.put("release", Build.VERSION.RELEASE)

      invoke.resolve(ret)
    }

    private fun isAHQStorePackage(pkg: String): Boolean {
      val packageManager = activity.packageManager

      val source = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
        packageManager.getInstallSourceInfo(pkg).initiatingPackageName
      } else {
        packageManager.getInstallerPackageName(pkg)
      }

      return source == activity.applicationInfo.packageName
    }

    @Command
    fun isAHQStorePackage(invoke: Invoke) {
      isAHQStorePackage(invoke.parseArgs(String::class.java))
    }

    @Command
    fun listInstalled(invoke: Invoke) {
      val pm = activity.packageManager
      val ret = JSObject()

      val pkgs = JSArray()

      for (pkg in pm.getInstalledPackages(PackageManager.GET_META_DATA)) {
        val pkg = pkg!!
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
           pm.getInstallSourceInfo(pkg.packageName).installingPackageName
        }

        if (pkg.applicationInfo != null) {
          pkgs.put(pkg)
        }
      }

      ret.put("apps", pkgs)

      invoke.resolve(ret)
    }

    @Command
    fun showCode(invoke: Invoke) {
      val args = invoke.parseArgs(ShowCodeRequest::class.java)

      invoke.resolve()
    }

    @Command
    fun zoom(invoke: Invoke) {
      val args = invoke.parseArgs(ZoomRequest::class.java)

      val currentZoom = webView!!.scale
      webView!!.zoomBy(args.zoom / currentZoom)

      invoke.resolve()
    }

    private suspend fun installInner(invoke: Invoke) {
      val path = invoke.parseArgs(Data::class.java).data

      val apk = File(path)

      val pkgMan = activity.packageManager

      if (pkgInstaller == null) {
        pkgInstaller = PackageInstaller.getInstance(activity.baseContext)
      }
      if (!pkgMan.canRequestPackageInstalls()) {
        activity.startActivity(
          Intent(Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES)
            .setData(Uri.parse(String.format("package:%s", activity.baseContext.packageName))),
        )
      }

      val ret = JSObject()

      ret.put("success", false)
      if (apk.exists()) {
        try {
          val apkUri: Uri = FileProvider.getUriForFile(
            activity.baseContext,
            activity.applicationContext.packageName + ".fileprovider",
            apk
          )

          println("Got apk Uri")

          try {
            println("pkg installer")
            when (val result = pkgInstaller!!.createSession(InstallParameters(apkUri) {
              confirmation = Confirmation.IMMEDIATE
            }).await()) {
              is Session.State.Failed -> {
                println("Error $result")
                ret.put("msg", result.toString())
              }
              Session.State.Succeeded -> {
                ret.put("success", true)
                ret.put("msg", "Success")
              }
            }
          } catch (_: CancellationException) {
            println("Error Cancelled (u  s  e  r)")
            ret.put("msg", "The operation was cancelled")
          } catch (e: Exception) {
            println("Error $e")
            ret.put("msg", "Error: ${e.message}")
          }
        } catch (e: Throwable) {
          println("Error $e")
          ret.put("msg", e.message)
        }
      } else {
        ret.put("success", false)
        ret.put("msg", "The app path was not found!")
      }

      invoke.resolve(ret)
    }


    private suspend fun uninstallInner(invoke: Invoke) {
      val packageString = invoke.parseArgs(Data::class.java).data
      val resp = JSObject()

      resp.put("success", false)

      if (pkgUninstaller == null) {
        pkgUninstaller = PackageUninstaller.getInstance(activity.baseContext)
      }

      try {
        when (val res = pkgUninstaller!!.createSession(UninstallParameters(packageString) {
          confirmation = Confirmation.IMMEDIATE
        }).await()) {
          is Session.State.Failed -> {
            resp.put("msg", res.toString())
          }
          Session.State.Succeeded -> {
            resp.put("success", true)
            resp.put("msg", "Successful")
          }
        }
      } catch (e: Throwable) {
        resp.put("msg", e.message)
      }

      invoke.resolve(resp)
    }
}
