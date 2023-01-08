package org.example.entity;

import lombok.Data;

import javax.persistence.*;
import java.util.List;

@Entity
@Data
public class Result {

    @Id
    @GeneratedValue
    private Long id;
    @OneToMany(cascade = CascadeType.ALL)
    private List<ClassificationObjectStatistics> objectStatistics;
    private int k;
    private float splitRatio;
    private int testingSliceSize;
    private int trainingSliceSize;
    private float accuracy;
    private String algorithm;
}
