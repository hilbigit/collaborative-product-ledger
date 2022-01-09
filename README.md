# collaborative-product-ledger

First draft - work in progress...

Idea: Build a Radix DLT Blueprint for developing products with collaborative ownership of commercial profits according to individual "product contributions" (e.g. "time" spent on a project).

## Usage
The Scypto component CollaborativeProduct can be used to define a smart contract for this kind of ownership.

- Creates an NFT token called "ProductContributionTime", symbol "PCT"
- The function book_product_contribution_time allows (if admin or product_contribution_badge present) to mint a PCT - token with following parameters:
-    amount: u32,
-    task_description: String,
-    external_reference: String

## Vision / next steps
- The amount is usually of type / unit "time in minutes" - this unit shall be defined as a parameter of the components
- Other "resource types" may by allowed, e.g. "ProductContributionResource" which is a generic token being "convertible" - to "ProductContributionTime
- The "universal" token "WorkingTime" (WT) shall be used instead of amount: u32
- The value of each contribution - token shall be convertible to WT somehow
- The "total contributions" to a product in terms of WT shall be calculated as sum of all converted contributions
- There shall be functions fn BookProductValue(...) and fn GetProductValueShare(...) for booking "real value earned + shared" (in arbitrary tokens e.g. in xEUR, XRD,...) with the product to another component vault and for transferring the "real value earned + shared"  to the accounts of the contributors. How much may transferred depends on how much WT the contributing account has contributed in relation to the "total contributions" 
