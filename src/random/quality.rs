
//https://crypto.stackexchange.com/questions/57936/how-to-evaluate-chi-squared-result
//https://en.wikibooks.org/wiki/Algorithm_Implementation/Pseudorandom_Numbers/Chi-Square_Test#C#

/*
	// Calculates the chi-square value for N positive integers less than r
	// Source: "Algorithms in C" - Robert Sedgewick - pp. 517
	// NB: Sedgewick recommends: "...to be sure, the test should be tried a few times,
	// since it could be wrong in about one out of ten times."
	public bool IsRandom(int[] randomNums, int r)
	{
		//Calculate the number of samples - N
		int N = randomNums.Length;

		//According to Sedgewick: "This is valid if N is greater than about 10r"
		if (N <= 10*r)
			return false;

		double N_r = N / r;
		double chi_square = 0;
		Hashtable HT;

		//PART A: Get frequency of randoms
		HT = this.RandomFrequency (randomNums);

		//PART B: Calculate chi-square - this approach is in Sedgewick
		double f;
		foreach (DictionaryEntry Item in HT)
		{
			f = (int)(Item.Value) - N_r;
			chi_square += Math.Pow(f, 2);
		}
		chi_square = chi_square / N_r;

		//PART C: According to Swdgewick: "The statistic should be within 2(r)^1/2 of r
		//This is valid if N is greater than about 10r"
		if((r - 2 * Math.Sqrt (r) <= chi_square) &  (r + 2 * Math.Sqrt (r) >= chi_square))
			return true;
		else
			return false;
	}

	//Gets the frequency of occurrence of a randomly generated array of integers
	//Output: A hashtable, key being the random number and value its frequency
	private Hashtable RandomFrequency(int[] randomNums)
	{
		Hashtable HT = new Hashtable();
		int N = randomNums.Length;

		for(int i = 0; i <= N-1; i++)
		{
			if (HT.ContainsKey(randomNums[i]))
				HT[randomNums[i]] = (int) HT[randomNums[i]] + 1;
			else
				HT[randomNums[i]] = 1;
		}
	
		return HT;
	}
*/