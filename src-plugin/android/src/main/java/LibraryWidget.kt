import android.content.Context
import androidx.glance.GlanceId
import androidx.glance.GlanceModifier
import androidx.glance.GlanceTheme
import androidx.glance.appwidget.GlanceAppWidget
import androidx.glance.appwidget.provideContent
import androidx.glance.text.Text
import androidx.compose.runtime.Composable
import androidx.glance.appwidget.lazy.LazyColumn
import androidx.glance.layout.Column
import androidx.glance.layout.fillMaxWidth

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
        Column {
            LazyColumn(modifier = GlanceModifier.fillMaxWidth()) {
                item {
                    Text("First")
                }
            }
        }
    }
}

//class AddButtonAction : ActionCallback {
//    override suspend fun onAction(
//        context: Context,
//        glanceId: GlanceId,
//        parameters: ActionParameters
//    ) {
//        // Handle your FAB click action here
//    }
//}