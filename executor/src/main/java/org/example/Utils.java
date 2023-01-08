package org.example;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.function.Consumer;

public class Utils {
    public static void getOutput(InputStream inputStream, Consumer<String> consumer) throws IOException {
        try(
                InputStreamReader inputStreamReader = new InputStreamReader(inputStream);
                BufferedReader bufferedReader = new BufferedReader(inputStreamReader);
        ) {
            bufferedReader.lines().forEachOrdered(consumer);
        }
    }
}
