// use crate::applicant::domain::errors::ApplicantError;
// use crate::applicant::domain::repository::CandidateRepository;
// use crate::evalprocess::application::candidate_dto::ApplicantDto;
//
// struct RegisterCandidateService<R: CandidateRepository> {
//     candidate_repository: R,
// }
//
// impl<R: CandidateRepository> RegisterCandidateService<R> {
//     fn new(candidate_repository: R) -> Self {
//         RegisterCandidateService {
//             candidate_repository,
//         }
//     }
//
//     fn register_candidate(candidate: ApplicantDto) -> Result<(), ApplicantError> {
//         // Validate the candidate
//         candidate.map_to_entity()?;
//         // Register it to a Database
//         Ok(())
//     }
// }
