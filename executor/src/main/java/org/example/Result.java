package org.example;

import java.util.Map;

public class Result {
    private int k;
    private float split_ratio;
    private int testing_slice_size;
    private int training_slice_size;
    private float accuracy;
    private Map<String, Float> precisions;
    private Map<String, Float> fallout;
    private Map<String, Float> recall;
    private Algorithm algorithm;

    public int getK() {
        return k;
    }

    public void setK(int k) {
        this.k = k;
    }

    public float getSplit_ratio() {
        return split_ratio;
    }

    public void setSplit_ratio(float split_ratio) {
        this.split_ratio = split_ratio;
    }

    public int getTesting_slice_size() {
        return testing_slice_size;
    }

    public void setTesting_slice_size(int testing_slice_size) {
        this.testing_slice_size = testing_slice_size;
    }

    public int getTraining_slice_size() {
        return training_slice_size;
    }

    public void setTraining_slice_size(int training_slice_size) {
        this.training_slice_size = training_slice_size;
    }

    public float getAccuracy() {
        return accuracy;
    }

    public void setAccuracy(float accuracy) {
        this.accuracy = accuracy;
    }

    public Map<String, Float> getPrecisions() {
        return precisions;
    }

    public void setPrecisions(Map<String, Float> precisions) {
        this.precisions = precisions;
    }

    public Map<String, Float> getFallout() {
        return fallout;
    }

    public void setFallout(Map<String, Float> fallout) {
        this.fallout = fallout;
    }

    public Map<String, Float> getRecall() {
        return recall;
    }

    public void setRecall(Map<String, Float> recall) {
        this.recall = recall;
    }

    public Algorithm getAlgorithm() {
        return algorithm;
    }

    public void setAlgorithm(Algorithm algorithm) {
        this.algorithm = algorithm;
    }
}
