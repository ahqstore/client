package com.ahqstore.app

import android.content.Context
import android.content.Intent
import android.widget.RemoteViews
import android.widget.RemoteViewsService

class UpdateDataService : RemoteViewsService() {
  override fun onGetViewFactory(intent: Intent): RemoteViewsFactory {
    return StackRemoteViewsFactory(this.applicationContext)
  }
}

class StackRemoteViewsFactory(private val context: Context) : RemoteViewsService.RemoteViewsFactory {
  private var items: MutableList<String> = mutableListOf()

  private fun initData() {
    items.clear()
    for (i in 1..10) {
      items += "ListView item $i"
    }
  } 
    
  override fun onCreate() {
    initData()
  }

  override fun onDataSetChanged() {
    initData()
  }

  override fun onDestroy() {}

  override fun getCount(): Int {
    return items.count()
  }

  override fun getViewAt(p0: Int): RemoteViews {
    val view = RemoteViews(
      context.packageName,
      R.layout.library
    )
    
    view.setTextViewText(R.id.textView, items[p0])
    return view
  }

  override fun getLoadingView(): RemoteViews? {
    return null
  }

  override fun getViewTypeCount(): Int {
    return 1
  }

  override fun getItemId(p0: Int): Long {
    return p0.toLong()
  }

  override fun hasStableIds(): Boolean {
    return false
  }

}