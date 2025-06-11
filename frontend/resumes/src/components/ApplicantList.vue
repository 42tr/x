<script setup lang="ts">
import { ref } from 'vue';
import ApplicantScore from './ApplicantScore.vue';
import type { Applicant } from '../types/applicant';

// Sample data - in a real app this would come from an API or props
const applicants = ref<Applicant[]>([
  {
    id: 1,
    name: 'Jane Doe',
    scores: {
      experience: 8,
      education: 9,
      interview: 7,
      technical: 9,
      cultural: 8
    }
  },
  {
    id: 2,
    name: 'John Smith',
    scores: {
      experience: 7,
      education: 6,
      interview: 8,
      technical: 7,
      cultural: 9
    }
  },
  {
    id: 3,
    name: 'Alex Johnson',
    scores: {
      experience: 9,
      education: 7,
      interview: 9,
      technical: 8,
      cultural: 7
    }
  }
]);

// Add a method to sort applicants by average score
const sortByScore = () => {
  applicants.value.sort((a, b) => {
    const aScore = Object.values(a.scores).reduce((sum, score) => sum + score, 0) / 5;
    const bScore = Object.values(b.scores).reduce((sum, score) => sum + score, 0) / 5;
    return bScore - aScore; // Sort in descending order
  });
};

// Sort on component mount
sortByScore();
</script>

<template>
  <div class="applicant-list">
    <h1>Applicant Assessments</h1>
    
    <div class="controls">
      <button @click="sortByScore" class="sort-button">
        Sort by Score
      </button>
    </div>
    
    <div v-if="applicants.length === 0" class="empty-state">
      No applicants found
    </div>
    
    <div v-else class="applicants">
      <ApplicantScore 
        v-for="applicant in applicants" 
        :key="applicant.id" 
        :applicant="applicant"
      />
    </div>
  </div>
</template>

<style scoped>
.applicant-list {
  max-width: 1000px;
  margin: 0 auto;
  padding: 1rem;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
  color: #2c3e50;
}

.controls {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 1.5rem;
}

.sort-button {
  background-color: var(--primary-color, #42b883);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: background-color 0.2s ease;
}

.sort-button:hover {
  background-color: var(--secondary-color, #35495e);
}

.empty-state {
  text-align: center;
  padding: 3rem;
  background-color: #f8f9fa;
  border-radius: 8px;
  color: #6c757d;
  font-size: 1.2rem;
}

.applicants {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

@media (prefers-color-scheme: dark) {
  h1 {
    color: #e9ecef;
  }
  
  .empty-state {
    background-color: #2a2a2a;
    color: #adb5bd;
  }
}
</style>