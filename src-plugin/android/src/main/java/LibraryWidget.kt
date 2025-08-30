import android.content.Context
import androidx.glance.GlanceId
import androidx.glance.GlanceModifier
import androidx.glance.GlanceTheme
import androidx.glance.layout.padding
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.provideContent
import androidx.glance.text.Text
import androidx.glance.appwidget.action.ActionCallback
import androidx.glance.appwidget.action.actionRunCallback
import androidx.glance.ImageProvider
import androidx.compose.ui.unit.dp
import androidx.glance.action.ActionParameters
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.glance.appwidget.components.CircleIconButton
import androidx.glance.appwidget.components.Scaffold
import androidx.glance.appwidget.components.TitleBar
import androidx.glance.appwidget.lazy.LazyColumn
import androidx.glance.background
import androidx.glance.layout.fillMaxWidth

import com.plugin.ahqstore.R

class LibraryWidget: GlanceAppWidget() {
    override suspend fun provideGlance(context: Context, id: GlanceId) {
        provideContent {
             LibraryWidgetContent()
        }
    }
}

@Composable
fun LibraryWidgetContent() {
    GlanceTheme {
        Scaffold(
            titleBar = {
                TitleBar(
                    title = "Updates",
                    startIcon = ImageProvider(R.drawable.favicon),
                    modifier = GlanceModifier.padding(horizontal = 4.dp, vertical = 2.dp),
                    actions = {
                        CircleIconButton(
                            onClick = actionRunCallback<AddButtonAction>(),
                            contentDescription = "Check",
                            modifier = GlanceModifier.padding(8.dp).background(Color(255,255,255,255)),
                            imageProvider = ImageProvider(R.drawable.update)
                        )
                    }
                )
            }
        ) {
            LazyColumn(
                modifier = GlanceModifier.fillMaxWidth()
            ) {
                item {
                    Text("First")
                }
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