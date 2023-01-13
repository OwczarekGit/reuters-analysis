package org.example.execution;

import lombok.RequiredArgsConstructor;
import org.example.*;
import org.example.entity.ClassificationObjectStatistics;
import org.example.entity.Result;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Executors;
import java.util.concurrent.ThreadPoolExecutor;
import java.util.concurrent.TimeUnit;
import java.util.stream.Collectors;

@Service
@RequiredArgsConstructor
public class ExecutionService {

    public static int threadCount;
    public static String dataFolder;
    public static String workingDirectoryPath;
    public static String classifierPath;

    @Value("${config.thread-count}")
    public void setClassifierPath(int thread_count){
        threadCount = thread_count;
    }

    @Value("${config.data-folder}")
    public void setDataFolderPath(String data_folder){
        dataFolder = data_folder;
    }

    @Value("${config.working-directory-path}")
    public void setWorkingDirectoryPath(String working_directory_path){
        workingDirectoryPath = working_directory_path;
    }

    @Value("${config.classifier-path}")
    public void setClassifierPath(String classifier_path){
        classifierPath = classifier_path;
    }

    private final ResultRepository resultRepository;

    public Result executeSingleSimulation(ClassificationParameters params) {
        ThreadPoolExecutor threadPoolExecutor = (ThreadPoolExecutor) Executors.newFixedThreadPool(threadCount);
        threadPoolExecutor.submit(new SingleClassifierExecution(
                params.getK(),
                params.getRatio(),
                dataFolder,
                params.getMultithreading(),
                params.getAlgorithm()
        ));
        threadPoolExecutor.shutdown();
        try {
            threadPoolExecutor.awaitTermination(Long.MAX_VALUE, TimeUnit.NANOSECONDS);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
        return processSingle();

    }

    public void executeMultipleSimulations() {
        TaskGenerationService taskGenerationService = new TaskGenerationService(
                List.of(
                        3,
                        5,
                        6,
                        9,
                        12,
                        16,
                        20,
                        24,
                        27,
                        30
                ),
                List.of(
                        0.4,
                        0.3,
                        0.5,
                        0.6,
                        0.8
                ),
                List.of(
                        Algorithm.MANHATTAN,
                        Algorithm.CHEBYSHEV,
                        Algorithm.EUCLIDEAN
                ),
                dataFolder
        );
        ThreadPoolExecutor threadPoolExecutor = (ThreadPoolExecutor) Executors.newFixedThreadPool(threadCount);
        taskGenerationService.generateTasks().forEach(threadPoolExecutor::submit);
        threadPoolExecutor.shutdown();
        try {
            threadPoolExecutor.awaitTermination(Long.MAX_VALUE, TimeUnit.NANOSECONDS);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
        processResults();

    }

    void processResults() {
        ResultService.getResults().forEach((resultDto) -> {

            List<ClassificationObjectStatistics> objectStatistics = resultDto.getPrecisions().entrySet().stream()
                    .map((entry) -> {
                        ClassificationObjectStatistics stats = new ClassificationObjectStatistics();
                        stats.setObjectName(entry.getKey());
                        stats.setPrecision(entry.getValue().doubleValue());
                        stats.setRecall(resultDto.getRecall().get(entry.getKey()).doubleValue());
                        stats.setFallout(resultDto.getFallout().get(entry.getKey()).doubleValue());
                        return stats;
                    })
                    .collect(Collectors.toList());

            Result result = new Result();
            result.setAlgorithm(resultDto.getAlgorithm().getValue());
            result.setObjectStatistics(objectStatistics);
            result.setAccuracy(resultDto.getAccuracy());
            result.setK(resultDto.getK());
            result.setSplitRatio(resultDto.getSplit_ratio());
            result.setTestingSliceSize(resultDto.getTesting_slice_size());
            result.setTrainingSliceSize(resultDto.getTraining_slice_size());
            result.setCreationDate(LocalDateTime.now());
            resultRepository.save(result);
        });
    }

    private Result processSingle() {

        ResultDto resultDto = ResultService.getResults().get(0);

            List<ClassificationObjectStatistics> objectStatistics = resultDto.getPrecisions().entrySet().stream()
                    .map((entry) -> {
                        ClassificationObjectStatistics stats = new ClassificationObjectStatistics();
                        stats.setObjectName(entry.getKey());
                        stats.setPrecision(entry.getValue().doubleValue());
                        stats.setRecall(resultDto.getRecall().get(entry.getKey()).doubleValue());
                        stats.setFallout(resultDto.getFallout().get(entry.getKey()).doubleValue());
                        return stats;
                    })
                    .collect(Collectors.toList());

            Result result = new Result();
            result.setAlgorithm(resultDto.getAlgorithm().getValue());
            result.setObjectStatistics(objectStatistics);
            result.setAccuracy(resultDto.getAccuracy());
            result.setK(resultDto.getK());
            result.setSplitRatio(resultDto.getSplit_ratio());
            result.setTestingSliceSize(resultDto.getTesting_slice_size());
            result.setTrainingSliceSize(resultDto.getTraining_slice_size());
            result.setCreationDate(LocalDateTime.now());
            ResultService.getResults().clear();
            return resultRepository.save(result);

    }

}
