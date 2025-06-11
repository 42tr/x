# Resume Assessment Tool

A Vue 3 application built with TypeScript and Vite for evaluating job applicants based on various assessment criteria. The application calculates and visualizes scores and grades for each applicant, helping recruiters and hiring managers make better-informed decisions.

## Features

- Display individual applicant assessment scores
- Calculate overall score and final grade
- Visual representation of skills using radar charts
- Sort applicants by their overall score
- Responsive design that works on mobile and desktop devices
- Support for both light and dark modes

## Project Structure

```
src/
├── assets/           # Static assets
├── components/       # Vue components
│   ├── ApplicantList.vue    # Displays all applicants
│   ├── ApplicantScore.vue   # Shows individual applicant scores
│   └── ScoreChart.vue       # Radar chart visualization
├── types/            # TypeScript type definitions
│   └── applicant.ts          # Applicant interfaces and helper functions
├── App.vue           # Main application component
├── main.ts           # Application entry point
└── style.css         # Global styles
```

## Assessment Criteria

Applicants are evaluated based on five key criteria:

1. **Experience**: Professional experience relevant to the position
2. **Education**: Educational qualifications and certifications
3. **Interview**: Performance during the interview process
4. **Technical**: Technical skills and knowledge assessment
5. **Cultural**: Cultural fit with the organization

Each criterion is scored on a scale of 1-10, with the final grade calculated from the average score:

| Average Score | Grade |
|---------------|-------|
| 9.0 - 10.0    | A+    |
| 8.0 - 8.9     | A     |
| 7.0 - 7.9     | B+    |
| 6.0 - 6.9     | B     |
| 5.0 - 5.9     | C+    |
| 4.0 - 4.9     | C     |
| 3.0 - 3.9     | D     |
| 0.0 - 2.9     | F     |

## Getting Started

### Prerequisites

- Node.js (v18 or later recommended)
- pnpm (recommended) or npm

### Installation

```bash
# Clone the repository
git clone <repository-url>

# Navigate to the project directory
cd resumes

# Install dependencies
pnpm install
```

### Development

```bash
# Start the development server
pnpm dev
```

### Build for Production

```bash
# Build the application
pnpm build

# Preview the production build
pnpm preview
```

## Extending the Application

### Adding New Applicants

To add new applicants, modify the `applicants` array in `ApplicantList.vue`:

```typescript
const applicants = ref<Applicant[]>([
  {
    id: 4, // Increment the ID
    name: 'New Applicant',
    scores: {
      experience: 8,
      education: 7,
      interview: 9,
      technical: 8,
      cultural: 9
    }
  },
  // ...existing applicants
]);
```

### Adding New Assessment Criteria

To add a new criterion:

1. Update the `Applicant` interface in `types/applicant.ts`
2. Modify the `calculateScore` function to include the new criterion
3. Update the `ApplicantScore.vue` component to display the new criterion
4. Adjust the `ScoreChart.vue` component to include the new axis

## Technologies Used

- [Vue 3](https://vuejs.org/) - Progressive JavaScript framework
- [TypeScript](https://www.typescriptlang.org/) - Typed JavaScript
- [Vite](https://vitejs.dev/) - Next-generation frontend tooling
