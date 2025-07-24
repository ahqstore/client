package com.plugin.ahqstore

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map

val Context.updateState: DataStore<Preferences> by preferencesDataStore(name = "updateprefs")

val AutoUpdatePrefs = stringPreferencesKey("AUTOUPDATE")
val AppsToUpdateList = stringPreferencesKey("AppsToUpdate")

enum class AutoUpdatePreference {
  Skip,
  Notify,
  UpdateOverWifiNonMetered,
  UpdateOverWifiMetered,
  UpdateOverMobileData
}

fun toString(pref: AutoUpdatePreference): String {
  return when (pref) {
    AutoUpdatePreference.Skip -> "Skip"
    AutoUpdatePreference.Notify -> "Notify"
    AutoUpdatePreference.UpdateOverWifiNonMetered -> "UpdateOverWifiNonMetered"
    AutoUpdatePreference.UpdateOverWifiMetered -> "UpdateOverWifiMetered"
    AutoUpdatePreference.UpdateOverMobileData -> "UpdateOverMobileData"
  }
}

fun fromString(value: String): AutoUpdatePreference {
  return when (value) {
    "Skip" -> AutoUpdatePreference.Skip
    "Notify" -> AutoUpdatePreference.Notify
    "UpdateOverWifiNonMetered" -> AutoUpdatePreference.UpdateOverWifiNonMetered
    "UpdateOverWifiMetered" -> AutoUpdatePreference.UpdateOverWifiMetered
    "UpdateOverMobileData" -> AutoUpdatePreference.UpdateOverMobileData
    else -> AutoUpdatePreference.Skip
  }
}

class UpdatePreferencesState() {
  private lateinit var ctx: Context;

  constructor(ctx: Context) : this() {
    this.ctx = ctx
  }

  suspend fun getAutoUpdatePreference(): AutoUpdatePreference {
    return this.ctx.updateState.data.map { data ->
      val ret = fromString(data[AutoUpdatePrefs] ?: "Skip")

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
      AutoUpdatePreference.Skip -> false
      AutoUpdatePreference.Notify -> true
      AutoUpdatePreference.UpdateOverWifiNonMetered -> isWifi() && !isWifiMetered()
      AutoUpdatePreference.UpdateOverWifiMetered -> isWifi()
      AutoUpdatePreference.UpdateOverMobileData -> isCellular() || isWifi()
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

  private fun isCellular(): Boolean {
    try {
      val man = this.ctx.getSystemService(ConnectivityManager::class.java)!!

      val network = man.activeNetwork
      val capability = man.getNetworkCapabilities(network)

      return capability!!.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR)
    } catch (e: Exception) {
      return false;
    }
  }

  private fun isWifiMetered(): Boolean {
    try {
      val man = this.ctx.getSystemService(ConnectivityManager::class.java)!!

      val network = man.activeNetwork
      val capability = man.getNetworkCapabilities(network)!!

      return capability.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) && !capability.hasCapability(NetworkCapabilities.NET_CAPABILITY_NOT_METERED)
    } catch (e: Exception) {
      return false;
    }
  }

  suspend fun shallUpdateRun(): Boolean {
    var ret = false

    when (this.getAutoUpdatePreference()) {
      AutoUpdatePreference.Skip -> false
      else -> false
    }


    return ret
  }

  suspend fun setAutoUpdatePreference(pref: AutoUpdatePreference) {
    this.ctx.updateState.edit { data ->
      val ret = toString(pref)

      data[AutoUpdatePrefs] = ret;
    }
  }
}