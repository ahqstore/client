import android.content.Context
import android.util.Log
import androidx.glance.GlanceId
import androidx.glance.GlanceModifier
import androidx.glance.GlanceTheme
import androidx.glance.layout.Alignment
import androidx.glance.layout.Box
import androidx.glance.layout.padding
import androidx.glance.layout.size
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.provideContent
import androidx.glance.text.Text
import androidx.glance.appwidget.action.ActionCallback
import androidx.glance.appwidget.action.actionRunCallback
import androidx.glance.Image
import androidx.glance.ImageProvider
import androidx.glance.action.clickable
import androidx.compose.ui.unit.dp
import androidx.glance.action.ActionParameters
import androidx.glance.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.glance.appwidget.components.Scaffold

import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

class LibraryWidget: GlanceAppWidget() {
    override suspend fun provideGlance(context: Context, id: GlanceId) {
        provideContent {
            GlanceTheme {
                LibraryWidgetContent()
            }
        }
    }
}

@Composable
fun LibraryWidgetContent() {
    val time = LocalDateTime.now()!!
    val fmt = DateTimeFormatter.ofPattern("HH:mm:ss")
    val fmtTime = fmt.format(time)

    Log.d("GLANCE", "Glance Used")

    Scaffold {
        Box(
            modifier = GlanceModifier.fillMaxSize()
        ) {
            Text("Last Updated $fmtTime", modifier = GlanceModifier.padding(16.dp))

            Box(
                modifier = GlanceModifier.size(56.dp)
                    .padding(bottom = 16.dp, end = 16.dp)
                    .clickable(actionRunCallback<AddButtonAction>()),
                contentAlignment = Alignment.BottomEnd
            ) {
                Image(
                    provider = ImageProvider(android.R.drawable.ic_input_add),
                    contentDescription = "Add Item"
                )
            }
        }
    }
}

class AddButtonAction : ActionCallback {
    override suspend fun onAction(
        context: Context,
        glanceId: GlanceId,
        parameters: ActionParameters
    ) {
        // Handle your FAB click action here
    }
}