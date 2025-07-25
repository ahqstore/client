package com.plugin.ahqstore

import android.annotation.SuppressLint
import android.content.Context
import android.net.Uri
import android.os.Build
import androidx.annotation.ChecksSdkIntAtLeast
import androidx.annotation.RequiresApi
import ru.solrudev.ackpine.installer.PackageInstaller
import ru.solrudev.ackpine.installer.createSession
import ru.solrudev.ackpine.installer.parameters.InstallMode
import ru.solrudev.ackpine.installer.parameters.InstallerType
import ru.solrudev.ackpine.session.Session
import ru.solrudev.ackpine.session.await
import ru.solrudev.ackpine.session.parameters.Confirmation
import ru.solrudev.ackpine.uninstaller.PackageUninstaller
import ru.solrudev.ackpine.uninstaller.createSession

class InstallerHelper(ctx: Context) {
  private val installer = PackageInstaller.getInstance(ctx);
  private val uninstaller = PackageUninstaller.getInstance(ctx)

  suspend fun install(apk: Uri): Boolean {
    val out = installer.createSession(apk) {
      confirmation=Confirmation.IMMEDIATE
      installerType=InstallerType.SESSION_BASED
      requireUserAction=true
    }.await()

    return when(out) {
      Session.State.Succeeded -> true
      else -> false
    }
  }

  @ChecksSdkIntAtLeast(api = Build.VERSION_CODES.S)
  fun canSilentUpdate(): Boolean {
    return Build.VERSION.SDK_INT >= Build.VERSION_CODES.S
  }

  suspend fun selfUpdate(apk: Uri): Boolean {
    val out = installer.createSession(apk) {
      confirmation = Confirmation.IMMEDIATE
      installerType = InstallerType.SESSION_BASED
      installMode = InstallMode.InheritExisting(packageName = "com.ahqstore.app", dontKillApp = false)
      requireUserAction = true
    }.await()

    return when (out) {
      Session.State.Succeeded -> true
      else -> false
    }
  }

  @SuppressLint("NewApi")
  suspend fun update(pkg: String, apk: Uri): Boolean {
    return if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
      updateSilent(pkg, apk)
    } else {
      updateBelow31(pkg, apk)
    }
  }

  private suspend fun updateBelow31(pkg: String, apk: Uri): Boolean {
    val out = installer.createSession(apk) {
      confirmation = Confirmation.IMMEDIATE
      installerType = InstallerType.SESSION_BASED
      installMode = InstallMode.InheritExisting(packageName = pkg, dontKillApp = true)
      requireUserAction = true
    }.await()

    return when (out) {
      Session.State.Succeeded -> true
      else -> false
    }
  }

  @RequiresApi(31)
  suspend fun updateSilent(pkg: String, apk: Uri): Boolean {
    val out = installer.createSession(apk) {
      confirmation = Confirmation.IMMEDIATE
      installerType = InstallerType.SESSION_BASED
      installMode = InstallMode.InheritExisting(packageName = pkg, dontKillApp = true)
      requireUserAction = false
    }.await()

    return when (out) {
      Session.State.Succeeded -> true
      else -> false
    }
  }

  suspend fun uninstall(pkg: String): Boolean {
    val out = uninstaller.createSession(pkg) {
      confirmation=Confirmation.IMMEDIATE
    }.await()

    return when(out) {
      Session.State.Succeeded -> true
      else -> false
    }
  }
}