package org.example;

import java.util.ArrayList;
import java.util.List;

public class ResultService {

    private final static List<Result> results = new ArrayList<>();

    public static synchronized void addResult(Result result) {
        results.add(result);
    }
}
