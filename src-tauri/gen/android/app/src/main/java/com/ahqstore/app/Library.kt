package com.ahqstore.app

import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.Context
import android.content.Intent
import android.net.Uri
import android.view.View
import android.widget.RemoteViews

class Library : AppWidgetProvider() {
  override fun onUpdate(
    context: Context,
    appWidgetManager: AppWidgetManager,
    appWidgetIds: IntArray
  ) {
    // There may be multiple widgets active, so update all of them
    for (appWidgetId in appWidgetIds) {
      updateAppWidget(context, appWidgetManager, appWidgetId)
    }
  }

  override fun onEnabled(context: Context) {
    
  }

  override fun onDisabled(context: Context) {
    
  }
}

internal fun updateAppWidget(
  context: Context,
  appWidgetManager: AppWidgetManager,
  appWidgetId: Int
) {
  val widgetText = context.getString(R.string.appwidget_text)
  
  val intent = Intent(context, UpdateDataService::class.java).apply {
    putExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, appWidgetId)
    data = Uri.parse(toUri(Intent.URI_INTENT_SCHEME))
  }

  val views = RemoteViews(context.packageName, R.layout.library).apply {
    // Set up the RemoteViews object to use a RemoteViews adapter.
    // This adapter connects to a RemoteViewsService through the
    // specified intent.
    // This is how you populate the data.
    setRemoteAdapter(R.id.list, intent)

    
    setEmptyView(R.id.list, R.id.up_to_date)
  }

  // Do additional processing specific to this widget.

  appWidgetManager.updateAppWidget(appWidgetId, views)
}