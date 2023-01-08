package org.example;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.concurrent.Executors;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;

public class Main {
    public static void main(String[] args) {

        ThreadPoolExecutor threadPoolExecutor = (ThreadPoolExecutor) Executors.newFixedThreadPool(7);
        addTasks(threadPoolExecutor);

//        threadPoolExecutor.execute(
//                new SingleClassifierExecution(
//                1,
//                0.0,
//                "",
//                false,Algorithm.EUCLIDEAN
//                )
//        );
    }

    static void addTasks(ThreadPoolExecutor threadPoolExecutor) {
        for(int i = 3; i < 30; i++) {
            threadPoolExecutor.submit(
                    new SingleClassifierExecution(
                            i,
                            0.7,
                            "test-data",
                            false,Algorithm.EUCLIDEAN
                    )
            );
        }
    }


}
