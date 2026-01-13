# ACTION PLAN - PHASE 3: ML-Driven Optimization Engine

**Reference:** REF:P3-ML  
**Timeline:** 5-7 days  
**Status:** NOT STARTED  
**Dependencies:** Phase 1 must be complete (Phase 2 can run in parallel)

---

## 📋 PHASE 3 OBJECTIVES

- [ ] Build LSTM earnings prediction model
- [ ] Implement convex optimization resource allocator
- [ ] Create anomaly detection system
- [ ] Build feature engineering pipeline
- [ ] Create FastAPI ML service
- [ ] Integrate ML predictions with orchestrator
- [ ] Achieve >85% prediction accuracy

---

## ✅ TASK CHECKLIST

### 3.1 PROJECT STRUCTURE

#### Task 3.1.1: ML Engine Directory Setup

- [ ] Create `src/ml/` subdirectories

  - [ ] `src/ml/models/` - ML model implementations
  - [ ] `src/ml/utils/` - Utility functions
  - [ ] `src/ml/api/` - FastAPI service
  - [ ] `src/ml/tests/` - Test suite

- [ ] Create `src/ml/__init__.py`
- [ ] Create `src/ml/models/__init__.py`
- [ ] Create `src/ml/utils/__init__.py`
- [ ] Create `src/ml/api/__init__.py`

**Deliverables:**

- ML engine structure ready
- All **init**.py files created
- Directory hierarchy proper

---

### 3.2 EARNINGS PREDICTOR MODEL

#### Task 3.2.1: Earnings Predictor Implementation

**File:** `src/ml/models/earnings_predictor.py`

- [ ] Implement `EarningsPredictor` class

  - [ ] LSTM architecture (2 LSTM layers, Dropout, Dense)
  - [ ] Input shape: (sequence_length=24, n_features)
  - [ ] 128 → 64 LSTM units
  - [ ] Dropout: 0.2 between layers
  - [ ] Output: Single earnings prediction

- [ ] Implement model building

  - [ ] Sequential model with proper layer configuration
  - [ ] Adam optimizer with learning rate 0.001
  - [ ] MSE loss, MAE and MAPE metrics

- [ ] Implement initialization
  - [ ] `__init__(protocol_name, sequence_length=24, model_dir="models")`
  - [ ] Setup StandardScaler for feature normalization
  - [ ] Initialize model directory

**Deliverables:**

- LSTM model architecture defined
- Model compiles without errors
- Scaler properly initialized

**Tests Required:**

- [ ] Test model instantiation
- [ ] Test model compilation
- [ ] Test input shape validation

---

#### Task 3.2.2: Sequence Preparation

- [ ] Implement `prepare_sequences()` method

  - [ ] Extract features and target from DataFrame
  - [ ] Scale features using StandardScaler
  - [ ] Create rolling window sequences
  - [ ] Return (X_sequences, y_targets) tuple

- [ ] Implement sequence validation
  - [ ] Verify minimum sequence length
  - [ ] Check for missing data
  - [ ] Handle edge cases

**Deliverables:**

- Sequence preparation functional
- Feature scaling working
- Data validation in place

**Tests Required:**

- [ ] Test sequence creation
- [ ] Test feature scaling
- [ ] Test edge case handling
- [ ] Test data validation

---

#### Task 3.2.3: Model Training

- [ ] Implement `train()` method

  - [ ] Input: DataFrame with earnings and features
  - [ ] Parameters: epochs, batch_size, validation_split
  - [ ] Use TimeSeriesSplit for validation (not random split)
  - [ ] Implement early stopping (patience=10)
  - [ ] Implement checkpoint saving
  - [ ] Implement learning rate reduction

- [ ] Implement training callbacks

  - [ ] EarlyStopping on validation loss
  - [ ] ModelCheckpoint for best weights
  - [ ] ReduceLROnPlateau for learning rate adjustment

- [ ] Implement history tracking
  - [ ] Log training loss and metrics
  - [ ] Save training history

**Deliverables:**

- Training pipeline complete
- Callbacks properly configured
- History tracking functional
- Models saved to disk

**Tests Required:**

- [ ] Test training on sample data
- [ ] Test callback execution
- [ ] Test model persistence
- [ ] Test training history

---

#### Task 3.2.4: Prediction & Forecasting

- [ ] Implement `predict_next_24h()` method

  - [ ] Input: recent_data (last sequence_length hours)
  - [ ] Validate minimum data requirement
  - [ ] Generate 24 sequential predictions
  - [ ] Each prediction uses recent data + previous predictions
  - [ ] Return list of 24 hourly predictions

- [ ] Implement validation
  - [ ] Check data completeness
  - [ ] Validate sequence length
  - [ ] Check for NaN/Inf values

**Deliverables:**

- 24-hour forecasting working
- Sequential prediction logic
- Validation in place

**Tests Required:**

- [ ] Test prediction generation
- [ ] Test data validation
- [ ] Test edge cases
- [ ] Test numerical stability

---

#### Task 3.2.5: Model Evaluation

- [ ] Implement `evaluate()` method

  - [ ] Input: test_df (test dataset)
  - [ ] Calculate MSE, MAE, MAPE
  - [ ] Calculate RMSE and R² score
  - [ ] Return metrics dictionary

- [ ] Implement evaluation logging
  - [ ] Log all metrics clearly
  - [ ] Compare to baseline

**Deliverables:**

- Evaluation metrics computed
- Logging functional
- Metrics comprehensive

**Tests Required:**

- [ ] Test evaluation calculation
- [ ] Test metric accuracy
- [ ] Test logging

---

#### Task 3.2.6: Model Persistence

- [ ] Implement `save()` method

  - [ ] Save Keras model to `.h5` file
  - [ ] Save StandardScaler to pickle
  - [ ] Save feature names to pickle
  - [ ] Log save operation

- [ ] Implement `load()` method
  - [ ] Load model from `.h5`
  - [ ] Load scaler from pickle
  - [ ] Load feature names
  - [ ] Validate all components
  - [ ] Log load operation

**Deliverables:**

- Model persistence complete
- Save/load operations verified

**Tests Required:**

- [ ] Test model saving
- [ ] Test model loading
- [ ] Test round-trip consistency

---

### 3.3 RESOURCE ALLOCATOR

#### Task 3.3.1: Optimization Problem Setup

**File:** `src/ml/models/resource_allocator.py`

- [ ] Implement `ResourceConstraints` dataclass

  - [ ] total_cpu_cores: f64
  - [ ] total_memory_gb: f64
  - [ ] total_bandwidth_mbps: f64
  - [ ] total_storage_gb: f64

- [ ] Implement `ProtocolRequirements` dataclass

  - [ ] cpu_per_unit: f64
  - [ ] memory_per_unit: f64
  - [ ] bandwidth_per_unit: f64
  - [ ] storage_per_unit: f64
  - [ ] min_allocation: f64 (0-1)
  - [ ] max_allocation: f64 (0-1)

- [ ] Implement `AllocationResult` dataclass
  - [ ] allocations: Dict[protocol, %]
  - [ ] expected_earnings: f64
  - [ ] resource_utilization: Dict[resource, %]
  - [ ] is_optimal: bool
  - [ ] solver_status: str

**Deliverables:**

- Data structures properly defined
- Type hints complete
- Validation ready

---

#### Task 3.3.2: CVXPY Optimization

- [ ] Implement `ResourceAllocator` class

  - [ ] Initialize with ResourceConstraints
  - [ ] Setup logging

- [ ] Implement `optimize_allocation()` method

  - [ ] Create optimization variables (allocation per protocol)
  - [ ] Define objective: maximize earnings
  - [ ] Define constraints:
    - [ ] CPU constraint
    - [ ] Memory constraint
    - [ ] Bandwidth constraint
    - [ ] Storage constraint
    - [ ] Per-protocol min/max bounds
    - [ ] General 0-1 bounds
  - [ ] Solve using CVXPY with ECOS solver
  - [ ] Extract and return results

- [ ] Implement result formatting
  - [ ] Convert to AllocationResult
  - [ ] Calculate resource utilization %
  - [ ] Log all information

**Deliverables:**

- Convex optimization implemented
- Solver properly configured
- Results comprehensive

**Tests Required:**

- [ ] Test optimization convergence
- [ ] Test constraint satisfaction
- [ ] Test result format
- [ ] Test infeasible scenarios

---

#### Task 3.3.3: Reallocation Decision Logic

- [ ] Implement `should_reallocate()` method

  - [ ] Calculate earnings improvement
  - [ ] Calculate improvement percentage
  - [ ] Measure allocation change magnitude
  - [ ] Account for reallocation costs
  - [ ] Return decision with analysis

- [ ] Implement decision criteria

  - [ ] Minimum 10% improvement threshold
  - [ ] Net benefit > $0.50/hour
  - [ ] Maximum change tolerance
  - [ ] Cost-benefit analysis

- [ ] Implement analysis logging
  - [ ] Log improvement metrics
  - [ ] Log decision reasoning
  - [ ] Log financial impact

**Deliverables:**

- Reallocation logic complete
- Decision criteria clear
- Analysis comprehensive

**Tests Required:**

- [ ] Test improvement calculation
- [ ] Test decision making
- [ ] Test edge cases
- [ ] Test cost analysis

---

#### Task 3.3.4: Reallocation Cost Calculation

- [ ] Implement `calculate_reallocation_cost()` method

  - [ ] Input: current allocation, new allocation, restart times
  - [ ] For each protocol with >5% change:
    - [ ] Calculate restart downtime
    - [ ] Calculate lost earnings during restart
    - [ ] Sum total costs
  - [ ] Return total reallocation cost

- [ ] Implement cost accounting
  - [ ] Track lost earnings
  - [ ] Track infrastructure costs
  - [ ] Return itemized cost breakdown

**Deliverables:**

- Cost calculation functional
- Realistic cost modeling
- Cost breakdown available

**Tests Required:**

- [ ] Test cost calculation
- [ ] Test edge cases
- [ ] Test zero-cost scenarios

---

### 3.4 ANOMALY DETECTION

#### Task 3.4.1: Anomaly Detector Implementation

**File:** `src/ml/models/anomaly_detector.py`

- [ ] Implement `AnomalyDetector` class

  - [ ] Isolation Forest algorithm
  - [ ] Configurable contamination rate (default 0.1)
  - [ ] Multi-feature detection

- [ ] Implement detection methods

  - [ ] `fit(data: DataFrame)` - Train detector
  - [ ] `predict(data: DataFrame)` - Detect anomalies
  - [ ] `decision_function()` - Get anomaly scores

- [ ] Implement features for detection
  - [ ] Earnings deviation from moving average
  - [ ] Resource usage spikes
  - [ ] Connection drops
  - [ ] Unusual pattern detection

**Deliverables:**

- Anomaly detection operational
- Configurable thresholds
- Scoring available

**Tests Required:**

- [ ] Test anomaly detection
- [ ] Test known anomalies
- [ ] Test normal patterns

---

#### Task 3.4.2: Alerting System

- [ ] Implement alert triggers

  - [ ] Earnings drop >30%
  - [ ] Unusual resource usage
  - [ ] Connection failures
  - [ ] Performance degradation

- [ ] Implement alert actions
  - [ ] Log alerts
  - [ ] Trigger notifications
  - [ ] Automatic recovery if possible

**Deliverables:**

- Alert system operational
- Recovery procedures in place

**Tests Required:**

- [ ] Test alert triggering
- [ ] Test alert logging
- [ ] Test recovery actions

---

### 3.5 FEATURE ENGINEERING

#### Task 3.5.1: Feature Engineering Utilities

**File:** `src/ml/utils/feature_engineering.py`

- [ ] Implement time-series features

  - [ ] Lagged features (t-1, t-2, t-24)
  - [ ] Moving averages (1h, 6h, 24h)
  - [ ] Exponential moving averages
  - [ ] Rate of change
  - [ ] Volatility (rolling std dev)

- [ ] Implement domain features

  - [ ] Hour of day
  - [ ] Day of week
  - [ ] Time of month
  - [ ] Seasonal indicators

- [ ] Implement resource features
  - [ ] Current allocation %
  - [ ] Resource utilization %
  - [ ] Performance metrics
  - [ ] Health status

**Deliverables:**

- Feature engineering functions
- Feature documentation
- Example usage provided

**Tests Required:**

- [ ] Test feature calculation
- [ ] Test feature validity
- [ ] Test edge cases

---

#### Task 3.5.2: Feature Validation

- [ ] Implement feature validation

  - [ ] Check for NaN/Inf
  - [ ] Validate ranges
  - [ ] Check for missing values
  - [ ] Data type validation

- [ ] Implement feature scaling
  - [ ] Standardization (StandardScaler)
  - [ ] Normalization (MinMaxScaler)
  - [ ] Log transforms where appropriate

**Deliverables:**

- Feature validation complete
- Data quality checks
- Preprocessing pipeline

**Tests Required:**

- [ ] Test validation
- [ ] Test scaling
- [ ] Test invalid data handling

---

### 3.6 FASTAPI ML SERVICE

#### Task 3.6.1: API Initialization

**File:** `src/ml/api/main.py`

- [ ] Setup FastAPI application

  - [ ] Create FastAPI() instance
  - [ ] Configure CORS
  - [ ] Setup middleware
  - [ ] Configure logging

- [ ] Setup startup/shutdown events
  - [ ] Load models on startup
  - [ ] Initialize components
  - [ ] Cleanup on shutdown

**Deliverables:**

- FastAPI app initialized
- Middleware configured
- Lifecycle events working

---

#### Task 3.6.2: API Schemas

**File:** `src/ml/api/schemas.py`

- [ ] Define request/response schemas using Pydantic

  - [ ] PredictionRequest
  - [ ] PredictionResponse
  - [ ] AllocationRequest
  - [ ] AllocationResponse
  - [ ] EvaluationRequest
  - [ ] EvaluationResponse

- [ ] Add validation rules
  - [ ] Field validation
  - [ ] Range checking
  - [ ] Error messages

**Deliverables:**

- All schemas defined
- Validation rules
- Documentation in schemas

---

#### Task 3.6.3: Prediction Endpoint

- [ ] Implement POST `/predict/earnings/{protocol}`

  - [ ] Input: recent data (24 hours)
  - [ ] Output: 24-hour predictions
  - [ ] Error handling
  - [ ] Logging

- [ ] Implement POST `/predict/allocation`
  - [ ] Input: current allocations, earnings predictions
  - [ ] Output: optimized allocations
  - [ ] Include reallocation analysis
  - [ ] Error handling

**Deliverables:**

- Prediction endpoints functional
- Error handling
- Logging complete

**Tests Required:**

- [ ] Test prediction endpoint
- [ ] Test invalid input handling
- [ ] Test response format

---

#### Task 3.6.4: Training Endpoint

- [ ] Implement POST `/train/{protocol}`

  - [ ] Input: training data
  - [ ] Output: training results, metrics
  - [ ] Save trained model
  - [ ] Error handling

- [ ] Implement GET `/models`
  - [ ] List all trained models
  - [ ] Include model metadata
  - [ ] Training timestamps

**Deliverables:**

- Training endpoints functional
- Model management working

**Tests Required:**

- [ ] Test training endpoint
- [ ] Test model listing

---

#### Task 3.6.5: Health & Diagnostics

- [ ] Implement GET `/health`

  - [ ] Check all models loaded
  - [ ] Check database connection
  - [ ] Return service status

- [ ] Implement GET `/models/{protocol}/metrics`
  - [ ] Return latest metrics
  - [ ] Performance statistics
  - [ ] Last training time

**Deliverables:**

- Health checks functional
- Diagnostics available

---

### 3.7 ORCHESTRATOR INTEGRATION

#### Task 3.7.1: ML Engine Communication

**File:** `src/core/ml_client.rs`

- [ ] Create ML API client

  - [ ] HTTP requests to ML service
  - [ ] Error handling and retries
  - [ ] Response parsing

- [ ] Implement request methods
  - [ ] `get_earnings_predictions(protocol, data)`
  - [ ] `optimize_allocation(allocations, earnings)`
  - [ ] `detect_anomalies(metrics)`

**Deliverables:**

- ML client implemented
- All methods working
- Error handling complete

**Tests Required:**

- [ ] Test API calls
- [ ] Test error handling
- [ ] Test response parsing

---

#### Task 3.7.2: Optimization Loop

- [ ] Integrate into orchestrator

  - [ ] Collect earnings data (24 hours)
  - [ ] Generate predictions
  - [ ] Optimize allocation
  - [ ] Evaluate reallocation decision
  - [ ] Apply if beneficial

- [ ] Schedule optimization cycles
  - [ ] Run every 15 minutes
  - [ ] Track optimization results
  - [ ] Log decisions

**Deliverables:**

- Optimization loop functional
- Integration complete
- Scheduling working

**Tests Required:**

- [ ] Test optimization cycle
- [ ] Test decision making
- [ ] Test result application

---

### 3.8 TESTING & VALIDATION

#### Task 3.8.1: Unit Tests

- [ ] Test earnings predictor

  - [ ] Model creation
  - [ ] Training
  - [ ] Prediction
  - [ ] Evaluation

- [ ] Test resource allocator

  - [ ] Constraint setup
  - [ ] Optimization
  - [ ] Reallocation decisions

- [ ] Test anomaly detector

  - [ ] Detection
  - [ ] Alerting

- [ ] Test feature engineering

  - [ ] Feature calculation
  - [ ] Validation
  - [ ] Scaling

- [ ] Test API endpoints
  - [ ] Request/response formats
  - [ ] Error handling
  - [ ] Validation

**Target:** 85%+ code coverage

**Test Files:**

- [ ] `src/ml/tests/test_earnings_predictor.py`
- [ ] `src/ml/tests/test_resource_allocator.py`
- [ ] `src/ml/tests/test_anomaly_detector.py`
- [ ] `src/ml/tests/test_feature_engineering.py`
- [ ] `src/ml/tests/test_api.py`

**Deliverables:**

- All tests passing
- 85%+ coverage
- Edge cases covered

---

#### Task 3.8.2: Integration Tests

- [ ] Test end-to-end prediction pipeline

  - [ ] Data loading → Features → Prediction
  - [ ] Realistic data flows

- [ ] Test optimization pipeline

  - [ ] Data collection → Prediction → Optimization

- [ ] Test API integration
  - [ ] Multiple endpoint calls
  - [ ] Data consistency

**Test File:**

- [ ] `src/ml/tests/test_integration.py`

**Deliverables:**

- Integration tests passing
- End-to-end flows verified

---

#### Task 3.8.3: Performance Validation

- [ ] Prediction accuracy >85%

  - [ ] Test on historical data
  - [ ] Measure MAE, RMSE, MAPE
  - [ ] Compare to baselines

- [ ] API response time <500ms

  - [ ] Test with load
  - [ ] Monitor resource usage

- [ ] Optimization time <10s
  - [ ] Test with all protocols
  - [ ] Verify convergence

**Deliverables:**

- Performance targets met
- Benchmarks documented

---

### 3.9 DOCUMENTATION

#### Task 3.9.1: Model Documentation

- [ ] Document earnings predictor

  - [ ] Architecture explanation
  - [ ] Feature description
  - [ ] Training procedure
  - [ ] Usage examples

- [ ] Document resource allocator

  - [ ] Mathematical formulation
  - [ ] Constraint explanation
  - [ ] Usage examples

- [ ] Document feature engineering
  - [ ] Feature definitions
  - [ ] Calculation formulas
  - [ ] Usage examples

**Deliverables:**

- Model documentation complete
- All formulas clear
- Examples provided

---

#### Task 3.9.2: API Documentation

- [ ] Document all endpoints

  - [ ] Request/response schemas
  - [ ] Error codes
  - [ ] Usage examples
  - [ ] Integration guides

- [ ] Create API reference guide
- [ ] Add to `docs/API.md`

**Deliverables:**

- API fully documented
- Examples comprehensive

---

## 📊 PHASE 3 SUCCESS CRITERIA

**All of the following must be true:**

- [ ] LSTM earnings predictor implemented
- [ ] Prediction accuracy >85% on historical data
- [ ] Resource allocator using convex optimization
- [ ] Anomaly detection operational
- [ ] Feature engineering pipeline complete
- [ ] FastAPI ML service fully functional
- [ ] All API endpoints working
- [ ] 85%+ code coverage
- [ ] All tests passing
- [ ] Performance targets met
- [ ] Integration with orchestrator complete
- [ ] Comprehensive documentation
- [ ] All changes committed to Git

---

## 🚀 NEXT PHASE

Once Phase 3 is complete and approved:
→ **Phase 4: Core Orchestrator Engine** (REF:P4-ORCH)

---

## 📝 DEVELOPMENT NOTES

- Use TensorFlow/Keras for neural networks
- Use CVXPY with ECOS solver for optimization
- Use scikit-learn for anomaly detection
- FastAPI for async API service
- pytest for all Python testing
- Mock external APIs in tests
- Focus on numerical stability
- Document all mathematical formulas

---

**Last Updated:** January 13, 2026  
**Status:** READY FOR EXECUTION
