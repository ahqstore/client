package com.ahqstore.alternate

import android.content.Context
import android.net.nsd.NsdManager
import android.net.nsd.NsdServiceInfo
import android.os.Build
import android.os.ext.SdkExtensions
import android.util.Log
import dadb.AdbKeyPair
import dadb.Dadb
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.FlowPreview
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.flow.flatMapConcat
import kotlinx.coroutines.flow.timeout
import kotlin.time.Duration.Companion.seconds

data class AdbData(val host: String, val port: Int)

enum class AdbService(val type: String) {
    CONNECT("_adb-tls-connect._tcp."),
    PAIRING("_adb-tls-pairing._tcp.")
}

class AdbDetect(ctx: Context) {
    private val nsd: NsdManager = ctx.getSystemService(Context.NSD_SERVICE) as NsdManager;

    @OptIn(ExperimentalCoroutinesApi::class, FlowPreview::class)
    fun discover(serviceType: AdbService): Flow<AdbData> = callbackFlow {
        val discoveryListener = object : NsdManager.DiscoveryListener {
            override fun onServiceFound(service: NsdServiceInfo) {
                trySend(service)
            }

            override fun onDiscoveryStarted(regType: String) {}
            override fun onServiceLost(service: NsdServiceInfo) {}
            override fun onDiscoveryStopped(serviceType: String) {}
            override fun onStartDiscoveryFailed(serviceType: String, errorCode: Int) { close() }
            override fun onStopDiscoveryFailed(serviceType: String, errorCode: Int) { close() }
        }

        nsd.discoverServices(serviceType.type, NsdManager.PROTOCOL_DNS_SD, discoveryListener)

        awaitClose { nsd.stopServiceDiscovery(discoveryListener) }
    }.flatMapConcat { serviceInfo ->
        resolveServiceFlow(serviceInfo).timeout(3.seconds)
    }

    private fun resolveServiceFlow(service: NsdServiceInfo): Flow<AdbData> = callbackFlow {
        nsd.resolveService(service, object : NsdManager.ResolveListener {
            override fun onServiceResolved(resolved: NsdServiceInfo) {
                // TODO: Add stuff later
                trySend(AdbData(resolved.host.hostAddress ?: "", resolved.port))

                close()
            }

            override fun onResolveFailed(service: NsdServiceInfo, errorCode: Int) {
                close()
            }
        })
        awaitClose {  }
    }
}