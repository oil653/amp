package com.example.amp

import io.flutter.embedding.android.FlutterActivity

class MainActivity : FlutterActivity() {
    init {
        System.loadLibrary("hub")
    }
}
