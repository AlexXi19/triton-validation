/**
 * 
 * The job config needs:
 * 
 * Job: 
 * - Job name
 * - Job ID	
 * - Priority
 * 
 * Model: 
 * - Model Name 
 * - Sample input (Or default generated from triton config)
 * - Sample output (Or default generated from triton config) 
 * - How to measure performance like accuracy, recall, etc.
 * 
 * Resource: 
 * - Resource constraints (CPU, Memory, GPU, etc.)
 * 
 * Load Test: 
 * - Load test configs
 *   - How much load? Not sure how much of this to expose to the user 
 * - If load test 
 */


/**
 * What the tests measure: 
 * 
 * ML Model Performance: 
 * - Accuracy
 * - Recall
 * - Precision
 * - F1
 * - AUC
 * - Confusion matrix
 * - ROC curve
 * 
 * System/Request performance: 
 * - Latency
 * - Throughput
 * - Error rate
 * 
 */