plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android") version "2.2.10"
    id("org.jetbrains.kotlin.plugin.compose") version "2.2.10"
    kotlin("plugin.serialization") version "2.2.20"
}

android {
    namespace = "com.plugin.ahqstore"
    compileSdk = 36

    defaultConfig {
        minSdk = 26

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }

    buildFeatures {
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = "1.5.15"
    }
}

dependencies {
    val ktor_version = "3.3.0"
    val ackpineVersion = "0.15.3"

    implementation("ru.solrudev.ackpine:ackpine-core:$ackpineVersion")
    implementation("ru.solrudev.ackpine:ackpine-ktx:$ackpineVersion")

    implementation("androidx.core:core-ktx:1.17.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.work:work-runtime-ktx:2.10.3")
    implementation("androidx.datastore:datastore-preferences:1.1.7")

    implementation("io.ktor:ktor-client-core:${ktor_version}")
    implementation("io.ktor:ktor-client-cio:${ktor_version}")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.9.0")

    // Get Full Compose
    implementation("androidx.compose.material3:material3")

    // For AppWidgets support
    implementation("androidx.glance:glance-appwidget:1.1.1")
    //  For interop APIs with Material 3
    implementation("androidx.glance:glance-material3:1.1.1")

    implementation("androidx.compose.ui:ui-tooling:1.9.0")
    debugImplementation("androidx.compose.ui:ui-tooling:1.9.0")

    implementation("com.google.android.material:material:1.12.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.3.0")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.7.0")
    implementation(project(":tauri-android"))
}
