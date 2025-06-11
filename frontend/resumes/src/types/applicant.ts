export interface Applicant {
  id: number;
  name: string;
  scores: {
    experience: number; // 工作经验
    education: number;  // 教育背景
    interview: number;  // 面试表现
    technical: number;  // 技术能力
    cultural: number;   // 文化契合度
  };
}

export interface ApplicantScore {
  score: number;  // 分数
  grade: string;  // 等级
}

// 计算应聘者的平均分数
export const calculateScore = (applicant: Applicant): number => {
  const { experience, education, interview, technical, cultural } = applicant.scores;
  return (experience + education + interview + technical + cultural) / 5;
};

// 根据分数计算等级
export const calculateGrade = (score: number): string => {
  if (score >= 9) return 'A+'; // 优秀
  if (score >= 8) return 'A';  // 优良
  if (score >= 7) return 'B+'; // 良好
  if (score >= 6) return 'B';  // 一般
  if (score >= 5) return 'C+'; // 及格
  if (score >= 4) return 'C';  // 勉强及格
  if (score >= 3) return 'D';  // 不及格
  return 'F';                  // 不合格
};