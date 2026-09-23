#[cfg(test)]
mod tests {
  use super::super::algorithm::run;
  use super::super::generator::generate_numeric_string;

  #[test]
  fn random() {
    let num1 = &generate_numeric_string(100);
    let num2 = &generate_numeric_string(100);
    let result_str = run(num1, num2);
    assert_ne!(result_str.len(), 0);
  }

  #[test]
  fn small_numbers() {
    let num1 = "1234";
    let num2 = "5678";
    let result_str = run(num1, num2);
    assert_eq!(result_str, "7006652");
  }

  #[test]
  fn large_numbers() {
    let num1 = "12365498744651894615198151985615498651895622849848942984983497249828465498524598423984298142984";
    let num2 = "97813975985391757983175978319835197594893817913944598319813978491389889137948139789318989173894";
    let result_str = run(num1, num2);
    assert_eq!(
      result_str,
      "1209518597256772349924912772301961125213274010608467104044047684526638372334746189115490854579240598100886616491021908054505326893828648519842689505725712365268981872514787211033864452059696"
    );
  }
}
