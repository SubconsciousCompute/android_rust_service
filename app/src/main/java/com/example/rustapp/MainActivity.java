package com.example.rustapp;

import android.content.Intent;
import android.os.Build;
import android.os.Bundle;

import androidx.appcompat.app.AppCompatActivity;
import androidx.appcompat.widget.SwitchCompat;

public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        setContentView(R.layout.activity_main);

        SwitchCompat serviceToggle = (SwitchCompat) findViewById(R.id.serviceToggle);
        serviceToggle.setOnCheckedChangeListener((compoundButton, isChecked) -> {
            Intent serviceIntent = new Intent(this, RustService.class);
            if (isChecked)
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
                    startForegroundService(serviceIntent);
                else
                    startService(serviceIntent);
            else
                stopService(serviceIntent);
        });
    }

}
