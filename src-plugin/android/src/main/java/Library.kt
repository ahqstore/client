import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.GlanceAppWidgetReceiver

class Library: GlanceAppWidgetReceiver() {
    override val glanceAppWidget: GlanceAppWidget
        get() = LibraryWidget()
}