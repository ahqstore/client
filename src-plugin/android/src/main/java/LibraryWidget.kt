import android.content.Context
import androidx.glance.GlanceId
import androidx.glance.GlanceTheme
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.components.Scaffold
import androidx.glance.appwidget.provideContent
import androidx.glance.text.Text

import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

class LibraryWidget: GlanceAppWidget() {
    override suspend fun provideGlance(context: Context, id: GlanceId) {
        val time = LocalDateTime.now()!!

        val fmt = DateTimeFormatter.ofPattern("HH:mm:ss")

        val fmtTime = fmt.format(time)

        provideContent {
            GlanceTheme {
                Scaffold {
                    Text("Last Updated $fmtTime")
                }
            }
        }
    }
}