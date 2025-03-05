import {ChangeEvent, FormEvent, useId, useState} from "react";

class ApplicantModel {
  name: string;
  lastname: string;
  documentId: string;

  constructor(name: string = '', lastname: string = '', documentId: string = '') {
    this.name = name;
    this.lastname = lastname;
    this.documentId = documentId;
  }

  isValid(): boolean {
    return this.name.trim() !== '' &&
      this.lastname.trim() !== '' &&
      this.documentId.trim() !== '';
  }

  getFullName(): string {
    return `${this.name} ${this.lastname}`;
  }

  reset(): void {
    this.name = '';
    this.lastname = '';
    this.documentId = '';
  }
}

function ApplicantForm() {
  const [applicant, setApplicant] = useState<ApplicantModel>(new ApplicantModel());
  const [errors, setErrors] = useState<Record<string, string>>({});
  const nameInputId = useId();
  const lastnameInputId = useId();
  const documentInputId = useId();
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    const {name, value} = e.target;
    setApplicant(prevState => {
      return new ApplicantModel(
        name === 'name' ? value : prevState.name,
        name === 'lastname' ? value : prevState.lastname,
        name === 'documentId' ? value : prevState.documentId
      );
    });

    if (errors[name]) {
      setErrors({...errors, [name]: ''});
    }
  };

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!applicant.name.trim()) {
      newErrors.name = 'El nombre es requerido';
    }

    if (!applicant.lastname.trim()) {
      newErrors.lastname = 'El apellido es requerido';
    }

    if (!applicant.documentId.trim()) {
      newErrors.documentId = 'El documento de identidad es requerido';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (validateForm()) {
      console.log('Form submitted:', applicant);
      console.log('Full name:', applicant.getFullName());
      // Here you would typically send the data to an API

      // Reset form after submission if needed
      // setApplicant(new ApplicantModel());
    }
  };

  return (
    <>
      <h1>📝 Registrar Postulante</h1>
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor={nameInputId}>Nombre:</label>
          <input
            type="text"
            id={nameInputId}
            value={applicant.name}
            onChange={handleChange}
          />
          {errors.name && <div className="error">{errors.name}</div>}
        </div>

        <div className="form-group">
          <label htmlFor={lastnameInputId}>Apellido:</label>
          <input
            type="text"
            id={lastnameInputId}
            value={applicant.lastname}
            onChange={handleChange}
          />
          {errors.lastname && <div className="error">{errors.lastname}</div>}
        </div>

        <div className="form-group">
          <label htmlFor={documentInputId}>Documento de Identidad:</label>
          <input
            type="text"
            id={documentInputId}
            value={applicant.documentId}
            onChange={handleChange}
          />
          {errors.documentId && <div className="error">{errors.documentId}</div>}
        </div>

        <button type="submit">Registrar</button>
      </form>
    </>
  );
}

export default ApplicantForm