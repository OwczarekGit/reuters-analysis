package org.example;

import com.fasterxml.jackson.databind.ObjectMapper;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import org.example.execution.ExecutionService;
import org.springframework.beans.factory.annotation.Value;


import java.io.File;
import java.io.IOException;

@Getter
@RequiredArgsConstructor
public class SingleClassifierExecution implements Runnable{



    private final Integer k;
    private final Double ratio;
    private final String dataPath;
    private final Boolean multithreading;
    private final Algorithm algorithm;
    private final StringBuilder result = new StringBuilder();
    private final ObjectMapper mapper = new ObjectMapper();


    void classify() {
        ProcessBuilder processBuilder;
        if (multithreading) {
            processBuilder = new ProcessBuilder(
                    ExecutionService.classifierPath,
                    dataPath,
                    "-k", k.toString(),
                    "-r", ratio.toString(),
                    "-a", algorithm.getValue(),
                    "-m"
            );
        } else {
            processBuilder = new ProcessBuilder(
                    ExecutionService.classifierPath,
                    dataPath,
                    "-k", k.toString(),
                    "-r", ratio.toString(),
                    "-a", algorithm.getValue()
            );
        }
        processBuilder.directory(new File(ExecutionService.workingDirectoryPath));
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
            ResultDto resultMapped = mapper.readValue(result, ResultDto.class);
            ResultService.addResult(resultMapped);
        } catch (IOException e) {
            throw new RuntimeException(e);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public void run() {
        classify();
    }
}
