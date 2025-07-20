package com.plugin.ahqstore

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.preferencesDataStore

val Context.dataStore: DataStore<Preferences> by preferencesDataStore(name = "updatestats")

val BusyStateKey = booleanPreferencesKey("BUSY")

class UpdateWorkerStore() {
  private lateinit var ctx: Context;

  constructor(ctx: Context) : this() {
    this.ctx = ctx
  }

  suspend fun start() {
    this.ctx.dataStore.edit { data ->
      data[BusyStateKey] = true
    }
  }

  suspend fun stop() {
    this.ctx.dataStore.edit { data ->
      data[BusyStateKey] = false
    }
  }
}