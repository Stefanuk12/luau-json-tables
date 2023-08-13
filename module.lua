-- // Dependencies
local HttpService = game:GetService("HttpService")

-- // Encodes a table into a proper format
local function TableEncode(tbl)
	assert(typeof(tbl) == "table", "Invalid table")

	-- // Iterate and check for any special types
	for i, v in pairs(tbl) do
		local v_type = typeof(v)
		if (v_type == "table") then
			tbl[i] = TableEncode(v)
		elseif (v_type == "Color3") then
			tbl[i] = {
				class = "Color3",
				data = {v.R, v.G, v.B}
			}
		elseif (v_type == "Vector2") then
			tbl[i] = {
				class = "Vector2",
				data = {v.X, v.Y}
			}
		elseif (v_type == "Vector3") then
			tbl[i] = {
				class = "Vector3",
				data = {v.X, v.Y, v.Z}
			}
		end
	end

	-- // Return the encoded table
	return tbl
end

-- // Encodes table to proper format and then string
local function JSONEncode(tbl)
	return HttpService:JSONEncode(TableEncode(tbl))
end

-- // Decodes the JSON string into a table
local function JSONDecode(str, should_warn)
	-- // Decode the JSON string
	local tbl = str
	if (typeof(str) == "string") then
		tbl = HttpService:JSONDecode(str)
	end
	assert(typeof(tbl) == "table", "Invalid JSON string")

	-- // Iterate and check for any special types
	for i, v in pairs(tbl) do
		-- // Make sure is a table
		local v_type = typeof(v)
		if (v_type ~= "table") then
			continue
		end

		-- // Make sure it has a class and data
		if not (v.class and v.data) then
			tbl[i] = JSONDecode(v)
			continue
		end

		-- // Check the class and parse the data
		if (v.class == "Color3") then
			tbl[i] = Color3.fromRGB(unpack(v.data))
		elseif (v.class == "Vector2") then
			tbl[i] = Vector2.new(unpack(v.data))
		elseif (v.class == "Vector3") then
			tbl[i] = Vector3.new(unpack(v.data))
		elseif (should_warn) then
			warn("unidentified class " .. v.class)
		end
	end

	-- // Return the decoded table
	return tbl
end