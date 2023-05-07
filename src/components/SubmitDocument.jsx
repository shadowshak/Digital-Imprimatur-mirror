import React, { useState } from "react";
import {
  Box,
  Button,
  FormControl,
  Grid,
  InputLabel,
  MenuItem,
  Select,
  Step,
  StepLabel,
  Stepper,
  TextField,
  Typography,
} from "@mui/material";
import ArticleIcon from "@mui/icons-material/Article";
import PictureAsPdfIcon from "@mui/icons-material/PictureAsPdf";
import SaveIcon from "@mui/icons-material/Save";
import ApartmentIcon from "@mui/icons-material/Apartment";

const NewDocumentForm = ({ onNext, onSave }) => (
  <Box sx={{ pt: 3, width: 700 }}>
    <Typography variant="h4" gutterBottom>
      New Document Submission
    </Typography>
    <Grid container spacing={3}>
      <Grid item xs={12}>
        <TextField label="Document Title" fullWidth variant="outlined" />
      </Grid>
      <Grid item xs={12}>
        <FormControl fullWidth>
          <InputLabel id="publisher-label">Publisher</InputLabel>
          <Select label="Publisher" variant="outlined">
            {/* Add the publishers here */}
            <MenuItem value="publisher1">Publisher 1</MenuItem>
            <MenuItem value="publisher2">Publisher 2</MenuItem>
          </Select>
        </FormControl>
      </Grid>
      <Grid item xs={12}>
        <TextField
          label="Comments (optional)"
          fullWidth
          multiline
          rows={4}
          variant="outlined"
        />
      </Grid>
      <Grid item xs={12}>
        <Button
          onClick={onNext}
          sx={{ borderRadius: 28 }}
          variant="contained"
          color="primary"
        >
          Next
        </Button>
        <Button
          onClick={onSave}
          sx={{ borderRadius: 28, ml: 2 }}
          variant="outlined"
        >
          Save Draft
        </Button>
      </Grid>
    </Grid>
  </Box>
);

const DocumentUploadForm = ({ onNext, onSave }) => (
  <Box sx={{ pt: 3, minWidth: 700 }}>
    <Typography variant="h4" gutterBottom>
      Document Upload
    </Typography>
    {/* Implement your document upload functionality here */}
    <Button
      onClick={onNext}
      sx={{ borderRadius: 28 }}
      variant="contained"
      color="primary"
    >
      Next
    </Button>
    <Button
      onClick={onSave}
      sx={{ borderRadius: 28, ml: 2 }}
      variant="outlined"
    >
      Save Draft
    </Button>
  </Box>
);

const ReviewAndSubmitForm = ({ onSubmit, onSave }) => (
  <Box sx={{ pt: 3, minWidth: 700 }}>
    <Typography variant="h4" gutterBottom>
      Review and Submit
    </Typography>
    {/* Display thumbnail images of the uploaded documents here */}
    <Box>
      <Typography variant="h6">Submission Info</Typography>
      <Box sx={{ display: "flex", alignItems: "center" }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item>
            <ArticleIcon />
          </Grid>
          <Grid item>
            <Typography>Submission Title</Typography>
          </Grid>
        </Grid>
      </Box>
      <Box sx={{ display: "flex", alignItems: "center" }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item>
            <PictureAsPdfIcon />
          </Grid>
          <Grid item>
            <Typography>PDF</Typography>
          </Grid>
        </Grid>
      </Box>
      <Box sx={{ display: "flex", alignItems: "center" }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item>
            <SaveIcon />
          </Grid>
          <Grid item>
            <Typography>File size</Typography>
          </Grid>
        </Grid>
      </Box>
      <Box sx={{ display: "flex", alignItems: "center" }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item>
            <ApartmentIcon />
          </Grid>
          <Grid item>
            <Typography>Submitter email</Typography>
          </Grid>
        </Grid>
      </Box>
    </Box>
    <Button
      onClick={onSubmit}
      sx={{ borderRadius: 28, mt: 3 }}
      variant="contained"
      color="primary"
    >
      Submit
    </Button>
    <Button
      onClick={onSave}
      sx={{ borderRadius: 28, ml: 2, mt: 3 }}
      variant="outlined"
    >
      Save Draft
    </Button>
  </Box>
);

const DocumentSubmissionStepper = ({ onClose }) => {
  const [activeStep, setActiveStep] = useState(0);

  const handleNext = () => setActiveStep(activeStep + 1);
  const handleSave = () => {
    // Implement your save draft functionality here
    console.log("Save draft");
    onClose();
  };

  const handleSubmit = () => {
    // Implement your submit functionality here
    console.log("Submit");
  };

  return (
    <Box sx={{ width: "100%", p: 2 }}>
      <Stepper activeStep={activeStep}>
        <Step>
          <StepLabel>New Document Submission</StepLabel>
        </Step>
        <Step>
          <StepLabel>Document Upload</StepLabel>
        </Step>
        <Step>
          <StepLabel>Review and Submit</StepLabel>
        </Step>
      </Stepper>
      {activeStep === 0 && (
        <NewDocumentForm onNext={handleNext} onSave={handleSave} />
      )}
      {activeStep === 1 && (
        <DocumentUploadForm onNext={handleNext} onSave={handleSave} />
      )}
      {activeStep === 2 && (
        <ReviewAndSubmitForm onSubmit={handleSubmit} onSave={handleSave} />
      )}
    </Box>
  );
};

export default DocumentSubmissionStepper;
