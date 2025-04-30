package com.ahqstore.app

import android.content.Context
import androidx.compose.runtime.Composable
import androidx.glance.GlanceId
import androidx.glance.GlanceTheme
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.GlanceAppWidgetReceiver
import androidx.glance.appwidget.provideContent
import androidx.glance.layout.Column
import androidx.glance.text.Text
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class Library : GlanceAppWidgetReceiver() {
  override val glanceAppWidget: GlanceAppWidget = LibraryWidget()
}

class LibraryWidget : GlanceAppWidget() {

  override suspend fun provideGlance(context: Context, id: GlanceId) {
      provideContent {
        GlanceTheme {
          MyContent()
        }
      
    }
  }
  
  @Composable
  private fun MyContent() {
    Column {
      Text("Hello World")
    }
  }
}