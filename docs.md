### Po wszytkich (jak bedzie za dużo to sie przytnie)

### Body
- Ilość słów
- Ilość znaków
- Ilość zdań
- Średnia dlugość zdania

### Title
- Ilość słów
- Ilość znaków

#### Możę
- czy ma tagi czy nie?
- Normalizacja?
- Ilość wspólnych słów

https://en.wikipedia.org/wiki/Cosine_similarity
https://github.com/GoogleCloudPlatform/tf-estimator-tutorials/tree/master/00_Miscellaneous/text-similarity-analysis
https://dkpro.github.io/dkpro-core/releas
es/1.9.2/apidocs/de/tudarmstadt/ukp/dkpro/core/io/reuters/ReutersDocument.html







USA -> USA
        tp,fp,fn,tn
Usa      1, 0, 0, 0
Germany  0, 0, 0, 1
Japan    0, 0, 0, 1


USA -> JAPAN (classification result)
        tp,fp,fn,tn
Usa      0, 0, 1, 0
Germany  0, 0, 0, 0
Japan    0, 1, 0, 0


Japan -> Germany

        tp,fp,fn,tn
Usa      0, 0, 0, 0
Germany  0, 1, 0, 0
Japan    0, 0, 1, 0


        tp,fp,fn,tn
Usa      0, 0, 0, 0
Germany  0, 0, 0, 0
Japan    0, 0, 0, 0


USA, Germany, Japan








