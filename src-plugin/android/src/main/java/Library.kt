import android.content.Context
import android.content.Intent
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.GlanceAppWidgetReceiver

class Library: GlanceAppWidgetReceiver() {
    override val glanceAppWidget: GlanceAppWidget
        get() = LibraryWidget()

    override fun onReceive(context: Context, intent: Intent) {
        super.onReceive(context, intent)

        if (intent.action == "ACTION_UPDATE_WIDGET") {

        }
    }
}