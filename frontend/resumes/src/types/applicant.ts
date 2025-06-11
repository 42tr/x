export interface Applicant {
  id: number;
  name: string;
  scores: {
    experience: number;
    education: number;
    interview: number;
    technical: number;
    cultural: number;
  };
}

export interface ApplicantScore {
  score: number;
  grade: string;
}

export const calculateScore = (applicant: Applicant): number => {
  const { experience, education, interview, technical, cultural } = applicant.scores;
  return (experience + education + interview + technical + cultural) / 5;
};

export const calculateGrade = (score: number): string => {
  if (score >= 9) return 'A+';
  if (score >= 8) return 'A';
  if (score >= 7) return 'B+';
  if (score >= 6) return 'B';
  if (score >= 5) return 'C+';
  if (score >= 4) return 'C';
  if (score >= 3) return 'D';
  return 'F';
};