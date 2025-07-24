package com.plugin.ahqstore

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.util.Log
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import androidx.work.CoroutineWorker
import androidx.work.WorkerParameters

class BackgroundUpdateWorker(ctx: Context, params: WorkerParameters): CoroutineWorker(ctx, params) {
  private val notificationChannelId = "UpdateNotifications"

  private fun createNotificationChannel()
  {

    val notificationChannel = NotificationChannel(
      notificationChannelId,
      "Update Notifications",
      NotificationManager.IMPORTANCE_DEFAULT,
    )

    val notificationManager: NotificationManager? =
      ContextCompat.getSystemService(
        applicationContext,
        NotificationManager::class.java)

    notificationManager?.createNotificationChannel(
      notificationChannel
    )
  }


  override suspend fun doWork(): Result {
    val store = UpdateWorkerStore(this.applicationContext)

    try {
      return run(store, false)
    } catch (_: Exception) {
      // If coroutine shuts down due to 10mins reached
      store.stop();
      return Result.failure();
    }
  }

  private suspend fun run(store: UpdateWorkerStore, runForced: Boolean): Result {
    createNotificationChannel()

    val pref = UpdatePreferencesState(this.applicationContext)

    if (!runForced || !pref.shallUpdateCheck()) {
      return Result.success();
    }

    store.start()

    // Work

    val notif = NotificationCompat.Builder(this.applicationContext, notificationChannelId)
      .setContentTitle("Updates")
      .setContentText("We have updated!!")
      .setSmallIcon(
        R.drawable.favicon,
      )
      .build()

    Log.i("Enqueued", "Sent notification")
    ContextCompat.getSystemService(
      applicationContext,
      NotificationManager::class.java
    )?.notify(1, notif)

    store.stop()

    return Result.success()
  }
}