export interface Result {
  id?: number;
  objectStatistics: ClassificationObjectStatistics[];
  k: number;
  splitRatio: number;
  testingSliceSize: number;
  trainingSliceSize: number;
  accuracy: number;
  algorithm: string;
  creationDate: string;
}

export interface ClassificationObjectStatistics {
  id?: number;
  objectName: string;
  recall: number;
  fallout: number;
  precision: number;
}
