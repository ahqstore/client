package com.plugin.ahqstore

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import app.tauri.annotation.InvokeArg
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map

val Context.updateState: DataStore<Preferences> by preferencesDataStore(name = "updateprefs")

val AutoUpdatePrefs = stringPreferencesKey("AUTOUPDATE")
val AppsToUpdateList = stringPreferencesKey("AppsToUpdate")

@InvokeArg
enum class AutoUpdatePreference {
  Never,
  CheckOnly,
  UpdateDuringUnmeteredWifi,
  UpdateDuringMeteredWifi,
  Always
}

fun toString(pref: AutoUpdatePreference): String {
  return when (pref) {
    AutoUpdatePreference.Never -> "Skip"
    AutoUpdatePreference.CheckOnly -> "Notify"
    AutoUpdatePreference.UpdateDuringUnmeteredWifi -> "UpdateOverWifiNonMetered"
    AutoUpdatePreference.UpdateDuringMeteredWifi -> "UpdateOverWifiMetered"
    AutoUpdatePreference.Always -> "UpdateOverMobileData"
  }
}

fun fromString(value: String): AutoUpdatePreference {
  return when (value) {
    "Skip" -> AutoUpdatePreference.Never
    "Notify" -> AutoUpdatePreference.CheckOnly
    "UpdateOverWifiNonMetered" -> AutoUpdatePreference.UpdateDuringUnmeteredWifi
    "UpdateOverWifiMetered" -> AutoUpdatePreference.UpdateDuringMeteredWifi
    "UpdateOverMobileData" -> AutoUpdatePreference.Always
    else -> AutoUpdatePreference.Never
  }
}

class UpdatePreferencesState() {
  private lateinit var ctx: Context;

  constructor(ctx: Context) : this() {
    this.ctx = ctx
  }

  suspend fun getAutoUpdatePreference(): AutoUpdatePreference {
    return this.ctx.updateState.data.map { data ->
      val ret = fromString(data[AutoUpdatePrefs] ?: "Notify")

      return@map ret;
    }.first()
  }

  fun listenableAppsToUpdate(): Flow<List<String>> {
    return this.ctx.updateState.data.map { data ->
      val ret = data[AppsToUpdateList] ?: ""

      return@map ret.split(",")
    }
  }

  suspend fun shallUpdateCheck(): Boolean {
    return when (this.getAutoUpdatePreference()) {
      AutoUpdatePreference.Never -> false
      else -> isConnectedToInternet()
    }
  }

  suspend fun shallUpdate(): Boolean {
    if (!isConnectedToInternet()) {
      return false
    }

    return when (this.getAutoUpdatePreference()) {
      AutoUpdatePreference.Never -> false
      AutoUpdatePreference.CheckOnly -> false
      AutoUpdatePreference.UpdateDuringUnmeteredWifi -> isWifi() && !isWifiMetered()
      AutoUpdatePreference.UpdateDuringMeteredWifi -> isWifi()
      AutoUpdatePreference.Always -> isWifi() || isCellular()
    }
  }

  private fun isWifi(): Boolean {
    try {
      val man = this.ctx.getSystemService(ConnectivityManager::class.java)!!

      val network = man.activeNetwork
      val capability = man.getNetworkCapabilities(network)

      return capability!!.hasTransport(NetworkCapabilities.TRANSPORT_WIFI)
    } catch (e: Exception) {
      return false;
    }
  }

  private fun getActiveNetworkCapabilities() : NetworkCapabilities? {
    val man = this.ctx.getSystemService(ConnectivityManager::class.java)

    val network = man.activeNetwork
    val capability = man.getNetworkCapabilities(network)

    return capability
  }

  fun isConnectedToInternet(): Boolean {
    val capability = getActiveNetworkCapabilities()

    return capability?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true
  }


  fun isCellular(): Boolean {
    try {
      val capability = getActiveNetworkCapabilities()

      return capability!!.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR)
    } catch (e: Exception) {
      return false;
    }
  }

  fun isWifiMetered(): Boolean {
    try {
      val capability = getActiveNetworkCapabilities()

      return !(capability?.hasCapability(NetworkCapabilities.NET_CAPABILITY_NOT_METERED) ?: true)
    } catch (e: Exception) {
      return false;
    }
  }

  suspend fun setAutoUpdatePreference(pref: AutoUpdatePreference) {
    this.ctx.updateState.edit { data ->
      val ret = toString(pref)

      data[AutoUpdatePrefs] = ret;
    }
  }
}