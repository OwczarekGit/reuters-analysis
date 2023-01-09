package org.example;

import lombok.RequiredArgsConstructor;
import org.example.entity.Result;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RequiredArgsConstructor
@RestController
@RequestMapping("api/result")
public class ResultController {

    private final ResultRepository resultRepository;

    @GetMapping
    public List<Result> getAllResults() {
        return this.resultRepository.findAll();
    }

    @DeleteMapping("{id}")
    public void deleteResult(@PathVariable Long id) {
        this.resultRepository.deleteById(id);
    }
}
