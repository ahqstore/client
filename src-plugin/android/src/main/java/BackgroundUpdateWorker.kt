package com.plugin.ahqstore

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.util.Log
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import androidx.work.CoroutineWorker
import androidx.work.ForegroundInfo
import androidx.work.WorkerParameters
import kotlinx.coroutines.delay
import kotlin.time.Duration

private const val notificationChannelId = "UpdateNotifications"

class BackgroundUpdateWorker(ctx: Context, params: WorkerParameters): CoroutineWorker(ctx, params) {
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

    createNotificationChannel()

    try {
      val available = check(this.applicationContext, store, false)

      if (available) {
        setForeground(startForeground())

        runUpdate()

        delay(Duration.parse("15m"))
      }

      return Result.success()
    } catch (_: Exception) {
      // If coroutine shuts down due to 10mins reached
      store.stop();
      return Result.failure();
    }
  }

  private fun startForeground(): ForegroundInfo {
    val ctx = this.applicationContext

    val notify = NotificationCompat.Builder(ctx, notificationChannelId)
      .setContentTitle("Updating")
      .setSmallIcon(R.drawable.favicon)
      .setOngoing(true)
      .setTicker("Updating")
      .setOnlyAlertOnce(true)
      .setContentText("Installing 20/30 applications")
      .build()

    return ForegroundInfo(8, notify)
  }
}

suspend fun check(ctx: Context, store: UpdateWorkerStore, runForced: Boolean): Boolean {
  val pref = UpdatePreferencesState(ctx)

  if (!runForced || !pref.shallUpdateCheck()) {
    return false;
  }

  store.start()

  // Work

  val notif = NotificationCompat.Builder(ctx, notificationChannelId)
    .setContentTitle("Updates")
    .setContentText("We have updated!!")
    .setSmallIcon(
      R.drawable.favicon,
    )
    .build()

  Log.i("Enqueued", "Sent notification")
  ContextCompat.getSystemService(
    ctx,
    NotificationManager::class.java
  )?.notify(1, notif)

  store.stop()

  val updateAvailable = false

  return updateAvailable
}

suspend fun runUpdate() {

}