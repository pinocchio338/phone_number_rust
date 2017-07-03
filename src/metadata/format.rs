use regex::Regex;

#[derive(Clone, Debug)]
pub struct Format {
	/// A regex that is used to match the national (significant) number. For
	/// example, the pattern "(20)(\d{4})(\d{4})" will match number "2070313000",
	/// which is the national (significant) number for Google London.
	///
	/// Note the presence of the parentheses, which are capturing groups what
	/// specifies the grouping of numbers.
	pub(crate) pattern: Regex,

	/// Specifies how the national (significant) number matched by pattern should
	/// be formatted.
	///
	/// Using the same example as above, format could contain "$1 $2 $3", meaning
	/// that the number should be formatted as "20 7031 3000".
	///
	/// Each $x are replaced by the numbers captured by group x in the regex
	/// specified by pattern.
	pub(crate) format: String,

	/// A regex that is used to match a certain number of digits at the beginning
	/// of the national (significant) number. When the match is successful, the
	/// accompanying pattern and format should be used to format this number. For
	/// example, if leading_digits="[1-3]|44", then all the national numbers
	/// starting with 1, 2, 3 or 44 should be formatted using the
	/// accompanying pattern and format.
	///
	/// The first leadingDigitsPattern matches up to the first three digits of the
	/// national (significant) number; the next one matches the first four digits,
	/// then the first five and so on, until the leadingDigitsPattern can uniquely
	/// identify one pattern and format to be used to format the number.
	///
	/// In the case when only one formatting pattern exists, no
	/// leading_digits_pattern is needed.
	pub(crate) leading_digits: Vec<Regex>,

	/// Specifies how the national prefix ($NP) together with the first group
	/// ($FG) in the national significant number should be formatted in the
	/// NATIONAL format when a national prefix exists for a certain country.
	///
	/// For example, when this field contains "($NP$FG)", a number from Beijing,
	/// China (whose $NP = 0), which would by default be formatted without
	/// national prefix as 10 1234 5678 in NATIONAL format, will instead be
	/// formatted as (010) 1234 5678; to format it as (0)10 1234 5678, the field
	/// would contain "($NP)$FG". Note $FG should always be present in this field,
	/// but $NP can be omitted. For example, having "$FG" could indicate the
	/// number should be formatted in NATIONAL format without the national prefix.
	///
	/// This is commonly used to override the rule specified for the territory in
	/// the XML file.
	///
	/// When this field is missing, a number will be formatted without national
	/// prefix in NATIONAL format. This field does not affect how a number is
	/// formatted in other formats, such as INTERNATIONAL.
	pub(crate) national_prefix: Option<String>,

	/// Specifies how any carrier code ($CC) together with the first group ($FG)
	/// in the national significant number should be formatted when
	/// formatWithCarrierCode is called, if carrier codes are used for a certain
	/// country.
	pub(crate) domestic_carrier: Option<String>,
}
