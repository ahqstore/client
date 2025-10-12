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
import android.util.Log
import android.view.Window
import android.view.WindowInsets
import android.webkit.WebView
import androidx.activity.OnBackPressedCallback
import androidx.appcompat.app.AppCompatActivity
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
import kotlinx.coroutines.cancel
import kotlinx.coroutines.launch

import java.io.File
import java.util.Vector
import java.util.concurrent.TimeUnit

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

fun windowInset(web: WebView, window: Window) {
  val rootView = window.decorView

  var top: Int
  var bottom: Int

  if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
    top = rootView.rootWindowInsets.getInsets(WindowInsets.Type.statusBars()).top
    bottom = rootView.rootWindowInsets.getInsets(WindowInsets.Type.navigationBars()).bottom
  } else {
    val insets = rootView.rootWindowInsets

    top = insets.systemWindowInsetTop
    bottom = insets.systemWindowInsetBottom
  }

  web.evaluateJavascript(
    """
      window.topMargin = $top
      window.bottomMargin = $bottom
    """.trimIndent()
  ) {

  }
}

@TauriPlugin
class AHQStorePlugin(private val activity: Activity): Plugin(activity) {
  private var webView: WebView? = null

  private val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())

  private val updatePref = UpdatePreferencesState(activity)
  private val updateWorkerState = UpdateWorkerStore(activity)
  private val installHelper = InstallerHelper(activity)

  override fun load(webView: WebView) {
    this.webView = webView

    val callback = object : OnBackPressedCallback(true) {
      override fun handleOnBackPressed() {
        webView.evaluateJavascript("""
          window.backPressed()
        """.trimIndent()) {

        }
      }
    }

    (activity as AppCompatActivity).onBackPressedDispatcher.addCallback(callback)

    windowInset(webView, activity.window)

    Log.w("Enqueued", "Periodic Work Running")

    if (!activity.shouldShowRequestPermissionRationale(Manifest.permission.POST_NOTIFICATIONS)) {
      if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
        activity.requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 0)
      }
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

    Log.w("Enqueued", "Periodic Work Registered")

    this.dataSync()
  }

  @Command
  fun close(i: Invoke) {
    activity.finish()

    i.resolve()
  }

  @Command
  fun getCommit(invoke: Invoke) {
    val ctx = this.activity.baseContext!!

    scope.launch {
      invoke.resolve(CommitInfo(ctx).getCommit())
    }
  }

  @Command
  fun updateCommit(invoke: Invoke) {
    val ctx = this.activity.baseContext!!

    scope.launch {
      invoke.resolve(CommitInfo(ctx).fetchUpdateCommit())
    }
  }

  @Command
  fun unload(invoke: Invoke) {
    Log.d("AHQStorePlugin", "Tauri @Command unload called. Cancelling pluginScope.")
    try {
      scope.cancel("Plugin is shutting down") // CRITICAL: Cancel the scope
      // Clear references to help garbage collection
      webView = null
    } catch (e: Exception) {
      Log.e("AHQStorePlugin", "Error during plugin unload/cancellation: ${e.message}", e)
    } finally {
      invoke.resolve() // Always resolve the invoke
    }
  }

  private fun dataSync() {
    scope.launch {
      updatePref.listenableAppsToUpdate().collect { data ->
        val ret = JSObject()

        ret.put("update", data)

        trigger("updateStat", ret)
      }
    }

    scope.launch {
      updateWorkerState.listenableIsBusy().collect { data ->
        val ret = JSObject()

        ret.put("update", data)
        trigger("workerCurrentStat", ret)
      }
    }
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
  fun update(invoke: Invoke) {
    scope.launch {
      val path = invoke.parseArgs(Data::class.java).data

      val apk = File(path)

      val pkgMan = activity.packageManager

      if (!pkgMan.canRequestPackageInstalls()) {
        activity.startActivity(
          Intent(Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES)
            .setData(Uri.parse(String.format("package:%s", activity.baseContext.packageName)))
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

          val archive = pkgMan.getPackageArchiveInfo(apk.path, 0)!!

          println("Got apk Uri")

          ret.put("success", installHelper.update(archive.packageName, apkUri))
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
  }

  @Command
  fun getUpdatePref(invoke: Invoke) {
    scope.launch {
      val ret = JSObject()
      ret.put("pref", updatePref.getAutoUpdatePreference())

      invoke.resolve(ret)
    }
  }

  @Command
  fun setUpdatePref(invoke: Invoke) {
    scope.launch {
      val toSet = invoke.parseArgs(String::class.java)

      val toSetFinal = fromString(toSet)

      updatePref.setAutoUpdatePreference(toSetFinal)

      val ret = JSObject()

      invoke.resolve(ret)
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

    if (!pkgMan.canRequestPackageInstalls()) {
      activity.startActivity(
        Intent(Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES)
          .setData(Uri.parse(String.format("package:%s", activity.baseContext.packageName)))
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

        ret.put("success", installHelper.install(apkUri))
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

    resp.put("success", installHelper.uninstall(packageString))
    invoke.resolve(resp)
  }
}
