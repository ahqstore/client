package com.plugin.ahqstore

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
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
    createNotificationChannel()

    val store = UpdateWorkerStore(this.applicationContext);

    store.start()

    // Work

    val notif = NotificationCompat.Builder(this.applicationContext, notificationChannelId)
      .setContentTitle("Updates")
      .setContentText("We have updated!!")
      .build()

    ContextCompat.getSystemService(
      applicationContext,
      NotificationManager::class.java
    )?.notify(1, notif);

    store.stop();

    return Result.success()
  }
}