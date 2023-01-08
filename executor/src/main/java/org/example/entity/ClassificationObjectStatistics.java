package org.example.entity;

import lombok.Data;
import lombok.NoArgsConstructor;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.Id;

@Entity
@NoArgsConstructor
@Data
public class ClassificationObjectStatistics {

    @Id
    @GeneratedValue
    private Long id;
    private String objectName;
    private Double recall;
    private Double fallout;
    private Double precision;
}
