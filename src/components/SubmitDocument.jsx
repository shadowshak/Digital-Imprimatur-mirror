import React, { useState } from "react";
import {
  Box,
  Button,
  Grid,
  Input,
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
import axios from "axios";

const sessionToken = sessionStorage.getItem("token");

const NewDocumentForm = ({ onNext, onSave }) => {
  const [formValues, setFormValues] = React.useState({
    name: "",
    author: "",
    description: "",
  });

  const handleSubmit = () => {
    const data = {
      token: sessionToken,
      name: formValues.name,
      author: formValues.author,
      description: formValues.description,
    };
    axios
      .post("http://localhost:3001/sub/create", data)
      .then((response) => {
        console.log(response.data);
        // handle successful response
      })
      .catch((error) => {
        console.log(error);
        // handle error response
      });
    onNext();
  };

  return (
    <Box sx={{ pt: 3, width: 700 }}>
      <Typography variant="h4" gutterBottom>
        New Document Submission
      </Typography>
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <TextField
            label="Document Title"
            fullWidth
            variant="outlined"
            value={formValues.name}
            onChange={(e) =>
              setFormValues({ ...formValues, name: e.target.value })
            }
          />
        </Grid>
        <Grid item xs={12}>
          <TextField
            label="Author"
            fullWidth
            variant="outlined"
            value={formValues.author}
            onChange={(e) =>
              setFormValues({ ...formValues, author: e.target.value })
            }
          />
        </Grid>
        <Grid item xs={12}>
          <TextField
            label="Description"
            fullWidth
            multiline
            rows={4}
            variant="outlined"
            value={formValues.description}
            onChange={(e) =>
              setFormValues({ ...formValues, description: e.target.value })
            }
          />
        </Grid>
        <Grid item xs={12}>
          <Button
            onClick={handleSubmit}
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
};

const DocumentUploadForm = ({ onNext, onSave }) => {
  const [file, setFile] = useState(null);
  const [submissionId, setSubmissionId] = useState(1);

  const handleFileChange = (event) => {
    setFile(event.target.files[0]);
  };

  const handleUpload = async () => {
    if (!file) {
      alert("Please select a file to upload");
      return;
    }

    const formData = new FormData();
    formData.append("document", file);
    formData.append("submission_id", submissionId);
    formData.append("token", sessionToken);

    try {
      const response = await axios.post(
        "http://localhost:3001/document/upload",
        formData,
        {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        }
      );

      console.log(
        "Document uploaded successfully, ID:",
        response.data.document_id
      );
      onNext();
    } catch (error) {
      console.error("Error uploading document:", error);
      alert("Document uploaded successfully");
    }
  };

  return (
    <Box sx={{ pt: 3, minWidth: 700 }}>
      <Typography variant="h4" gutterBottom>
        Document Upload
      </Typography>
      {/* Implement your document upload functionality here */}
      <Input
        type="file"
        onChange={handleFileChange}
        style={{ display: "block", marginBottom: "16px" }}
      />
      <Button
        onClick={handleUpload}
        sx={{ borderRadius: 28 }}
        variant="contained"
        color="primary"
      >
        Upload
      </Button>
      <Button
        onClick={onNext}
        sx={{ borderRadius: 28, ml: 2 }}
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
};

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
    //console.log("Submit");
    activeStep === 2 ? window.location.reload() : handleNext();
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
