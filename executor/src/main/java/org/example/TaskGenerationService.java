package org.example;

import java.util.ArrayList;
import java.util.List;

public class TaskGenerationService {
    private List<Integer> kValues;
    private List<Double> splitRatios;
    private List<Algorithm> algorithms;
    private String dataPath;

    public TaskGenerationService(
            List<Integer> kValues,
            List<Double> splitRatios,
            List<Algorithm> algorithms,
            String dataPath
    ) {
        this.kValues = kValues;
        this.splitRatios = splitRatios;
        this.algorithms = algorithms;
        this.dataPath = dataPath;
    }

    public List<SingleClassifierExecution> generateTasks() {
        List<SingleClassifierExecution> list = new ArrayList<>();
        algorithms.forEach((algorithm -> {
            kValues.forEach(k -> {
                splitRatios.forEach(ratio -> {
                    list.add(new SingleClassifierExecution(k, ratio, dataPath, false, algorithm));
                });
            });
        }));

        return list;
    }

}
