package org.example;

import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.File;
import java.io.IOException;

public class SingleClassifierExecution implements Runnable{

    private final ObjectMapper mapper;
    private final Integer k;
    private final Double ratio;
    private final String dataPath;
    private final Boolean multithreading;
    private final Algorithm algorithm;
    private StringBuilder result;

    public SingleClassifierExecution(
            Integer k,
            Double ratio,
            String dataPath,
            Boolean multithreading,
            Algorithm algorithm
    ) {
        this.k = k;
        this.ratio = ratio;
        this.dataPath = dataPath;
        this.multithreading = multithreading;
        this.algorithm = algorithm;
        this.result = new StringBuilder();
        mapper = new ObjectMapper();
    }

    void classify() {
        ProcessBuilder processBuilder = new ProcessBuilder(
                "/home/kacper/Documents/reuters-analysis/classifier/target/release/analiza-danych",
                dataPath,
                "-k", k.toString(),
                "-r", ratio.toString(),
                "-a", algorithm.getValue()
        );
        processBuilder.directory(new File("/home/kacper/Documents/reuters-analysis/classifier"));
        try {
            Process p = processBuilder.start();
            Utils.getOutput(
                    p.getInputStream(),
                    (l) -> {
                        result.append(l);
                        //System.out.println(l);
                    }
            );
            p.waitFor();
            String result = this.result.toString();
            Result resultMapped = mapper.readValue(result, Result.class);
            System.out.println(result);
            ResultService.addResult(resultMapped);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
    }

    public Integer getK() {
        return k;
    }

    public Double getRatio() {
        return ratio;
    }

    public String getDataPath() {
        return dataPath;
    }

    public Boolean getMultithreading() {
        return multithreading;
    }

    public Algorithm getAlgorithm() {
        return algorithm;
    }


    @Override
    public void run() {
        classify();
    }
}
