package org.example;

import lombok.Getter;
import lombok.Setter;

import java.util.Map;

@Getter
@Setter
public class ResultDto {
    private int k;
    private float split_ratio;
    private int testing_slice_size;
    private int training_slice_size;
    private float accuracy;
    private Map<String, Float> precisions;
    private Map<String, Float> fallout;
    private Map<String, Float> recall;
    private Algorithm algorithm;

}
