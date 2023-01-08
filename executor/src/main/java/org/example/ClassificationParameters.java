package org.example;

import lombok.Data;

@Data
public class ClassificationParameters {
    private Double ratio;
    private Integer k;
    private Algorithm algorithm;
}
