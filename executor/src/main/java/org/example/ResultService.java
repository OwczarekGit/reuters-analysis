package org.example;

import lombok.Getter;
import org.example.entity.Result;

import java.util.ArrayList;
import java.util.List;
public class ResultService {
    @Getter
    private final static List<ResultDto> results = new ArrayList<>();

    public static synchronized void addResult(ResultDto result) {
        results.add(result);
    }

}
