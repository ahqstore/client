package com.plugin.ahqstore

import android.content.Context
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
    val aBody = client.get("https://api.github.com/repos/ahqstore/repo_community/commits")
      .body<String>()

    val ahqstore = Json.decodeFromString<ReleaseDataList>(aBody)[0].sha

    val altBody = client.get("https://api.github.com/repos/ahqstore/repo_android/commits")
      .body<String>()

    val alt = Json.decodeFromString<ReleaseDataList>(altBody)[0].sha

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
    var commit = ctx.commitStore.data.map { data ->
      val ahqstore = data[MainCommitKey] ?: ""
      val alt = data[AltCommitKey] ?: ""
      val expires = data[ExpiresTime] ?: 0

      val now = System.currentTimeMillis()

      if (now >= expires) {
        return@map null
      }

      val obj = JSObject()

      obj.put("ahqstore", ahqstore)
      obj.put("alt", alt)

      return@map obj
    }.lastOrNull()

    if (commit == null) {
      commit = fetchUpdateCommit()
    }

    return commit
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
    }.lastOrNull()

    if (commit == null) {
      commit = fetchUpdateCommitKt()
    }

    return commit
  }
}