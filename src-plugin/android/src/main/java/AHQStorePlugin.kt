package com.plugin.ahqstore

import android.app.Activity
import android.webkit.WebView

import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class ShowCodeRequest {
  var value: String = ""
}

@InvokeArg
class ZoomRequest {
  var zoom: Float = 100.0F
}

@TauriPlugin
class AHQStorePlugin(private val activity: Activity): Plugin(activity) {
    private var webview: WebView? = null

    override fun load(webView: WebView) {
      webview = webView 
    }

    @Command
    fun showCode(invoke: Invoke) {
      val args = invoke.parseArgs(ShowCodeRequest::class.java)

      val ret = JSObject()
      invoke.resolve(ret)
    }

    @Command
    fun zoom(invoke: Invoke) {
      val args = invoke.parseArgs(ZoomRequest::class.java)

      val currentZoom = webview!!.getScale()
      webview!!.zoomBy(args.zoom / currentZoom)
      
      val ret = JSObject()
      invoke.resolve(ret)
    }
}
