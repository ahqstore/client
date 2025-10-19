package com.plugin.ahqstore

import android.content.Context
import android.util.Log
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.longPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import app.tauri.plugin.JSObject
import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.engine.cio.CIO
import io.ktor.client.plugins.UserAgent
import io.ktor.client.request.get
import kotlinx.coroutines.flow.firstOrNull
import kotlinx.coroutines.flow.last
import kotlinx.coroutines.flow.lastOrNull
import kotlinx.coroutines.flow.map
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

val Context.commitStore: DataStore<Preferences> by preferencesDataStore(name = "commitStore")

val MainCommitKey = stringPreferencesKey("main")
val AltCommitKey = stringPreferencesKey("alt")
val ExpiresTime = longPreferencesKey("expires")

@Serializable
data class ReleaseData(val sha: String)

typealias ReleaseDataList = List<ReleaseData>

data class Commit(val ahqstore: String, val alt: String)

class CommitInfo() {
  val json = Json {
    ignoreUnknownKeys = true
  }
  val expiresInMillis = 1000 * 60 * 15 // 15 mins

  private lateinit var ctx: Context
  private val client = HttpClient(CIO) {
    install(UserAgent) {
      agent = "AHQ Store Installer"
    }
  }

  constructor(ctx: Context) : this() {
    this.ctx = ctx
  }

  suspend fun fetchUpdateCommitKt() : Commit {
    Log.i("COM", "Sending get commit")

    val aBody = client.get("https://api.github.com/repos/ahqstore/repo_community/commits")
      .body<String>()

    val ahqstore = json.decodeFromString<ReleaseDataList>(aBody)[0].sha

    val altBody = client.get("https://api.github.com/repos/ahqstore/repo_android/commits")
      .body<String>()

    val alt = json.decodeFromString<ReleaseDataList>(altBody)[0].sha

    ctx.commitStore.edit { d ->
      d[MainCommitKey] = ahqstore
      d[AltCommitKey] = alt
      d[ExpiresTime] = System.currentTimeMillis() + expiresInMillis
    }

    return Commit(ahqstore, alt)
  }

  suspend fun fetchUpdateCommit() : JSObject {
    val (ahqstore, alt) = fetchUpdateCommitKt()

    val obj = JSObject()

    obj.put("ahqstore", ahqstore)
    obj.put("alt", alt)

    return obj
  }

  suspend fun getCommit() : JSObject {
    Log.i("COM", "Requesting state")

    var commit = ctx.commitStore.data.firstOrNull()

    val expires = commit?.get(ExpiresTime) ?: 0
    val now = System.currentTimeMillis()

    if (commit == null || now >= expires) {
      Log.d("COM", "Fetching")
      return fetchUpdateCommit()
    }

    val ahqstore = commit[MainCommitKey] ?: ""
    val alt = commit[AltCommitKey] ?: ""

    val ret = JSObject()

    ret.put("ahqstore", ahqstore)
    ret.put("alt", alt)

    return ret
  }

  suspend fun getCommitKt() : Commit {
    var commit = ctx.commitStore.data.map { data ->
      val ahqstore = data[MainCommitKey] ?: ""
      val alt = data[AltCommitKey] ?: ""
      val expires = data[ExpiresTime] ?: 0

      val now = System.currentTimeMillis()

      if (now >= expires) {
        return@map null
      }

      return@map Commit(ahqstore, alt)
    }.firstOrNull()

    if (commit == null) {
      commit = fetchUpdateCommitKt()
    }

    return commit
  }
}