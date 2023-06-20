package com.example.rustapp;

import android.content.Intent;
import android.os.Build;
import android.os.Bundle;
import android.view.View;
import android.widget.Button;

import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity implements View.OnClickListener {
    private Button start, stop;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        setContentView(R.layout.activity_main);

        start = (Button) findViewById(R.id.startButton);
        stop = (Button) findViewById(R.id.stopButton);

        start.setOnClickListener(this);
        stop.setOnClickListener(this);
    }

    public void onClick(View view) {
        if (view == start) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                startForegroundService(new Intent(this, RustService.class));
            } else {
                startService(new Intent(this, RustService.class));
            }
        } else if (view == stop) {
            stopService(new Intent(this, RustService.class));
        }
    }
}
