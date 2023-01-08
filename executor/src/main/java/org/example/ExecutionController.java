package org.example;

import lombok.RequiredArgsConstructor;
import org.example.execution.ExecutionService;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RequiredArgsConstructor
@RestController
@RequestMapping("api/execute")
public class ExecutionController {
    private final ExecutionService service;
    @PostMapping
    public void executeClassification(@RequestBody ClassificationParameters classificationParameters) {
        service.executeSingleSimulation(classificationParameters);
    }
}
